#![allow(unused)]
use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::{get, post},
    },
};
use common::{
    Result,
    error::ServiceError,
    tables::categories::{
        Category, CategoryAction, CategoryByIdAction, CategoryCreateRequest, CategoryGetResponse,
        CategoryParam, CategoryQuery, CategoryRepository, CategorySummary, CategoryUpdateRequest,
    },
};
use sqlx::postgres::PgRow;
#[cfg(test)]
mod tests;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct CategoryPathParam {
    pub agit_id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct CategoryIdPath {
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct CategoryByIdPathParam {
    pub id: i64,
    pub agit_id: i64,
}
#[derive(Clone, Debug)]
pub struct CategoryController {
    pool: sqlx::PgPool,
    repo: CategoryRepository,
}

impl CategoryController {
    async fn query(
        &self,
        auth: Option<Authorization>,
        param: CategoryQuery,
    ) -> Result<by_types::QueryResponse<CategorySummary>> {
        let user_id = match auth {
            Some(Authorization::Bearer { claims }) => {
                let res = claims.custom.get("id");
                let mut id = 0;
                if let Some(res) = res {
                    id = res.parse().unwrap_or_default();
                }
                id
            }
            _ => 0,
        };

        let mut total_count = 0;
        let items: Vec<CategorySummary> = CategorySummary::query_builder(user_id)
            .limit(param.size())
            .page(param.page())
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;
                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;
        Ok(by_types::QueryResponse { total_count, items })
    }

    async fn create(
        &self,
        auth: Option<Authorization>,
        agit_id: i64,
        CategoryCreateRequest { title, description }: CategoryCreateRequest,
    ) -> Result<Json<Category>> {
        if auth.is_none() {
            return Err(ServiceError::Unauthorized);
        }
        let category = self.repo.insert(agit_id, title, description).await?;
        tracing::debug!("create category: {:?}", category);
        Ok(Json(category))
    }

    async fn update(
        &self,
        auth: Option<Authorization>,
        id: i64,
        param: CategoryUpdateRequest,
    ) -> Result<Category> {
        if auth.is_none() {
            return Err(ServiceError::Unauthorized);
        }
        let category = self.repo.update(id, param.into()).await?;
        Ok(category)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<Category> {
        if auth.is_none() {
            return Err(ServiceError::Unauthorized);
        }
        self.repo
            .delete(id)
            .await
            .map_err(|_| ServiceError::NotFound)
    }
}

impl CategoryController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Category::get_repository(pool.clone());
        Self { repo, pool }
    }
    // FIXME:
    pub fn route(pool: sqlx::PgPool) -> Result<by_axum::axum::Router> {
        let ctrl = Self::new(pool);
        Ok(by_axum::axum::Router::new().route("/:id", get()))
    }
    pub async fn act_category(
        State(ctrl): State<Self>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(CategoryPathParam { agit_id }): Path<CategoryPathParam>,
        Json(body): Json<CategoryAction>,
    ) -> Result<json<Category>> {
        match body {
            CategoryAction::Create(param) => {
                let new_category = ctrl.create(auth, agit_id, param).await?;
                Ok(new_category)
            }
        }
    }

    pub async fn act_category_by_id(
        State(ctrl): State<Self>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(CategoryIdPath { id }): Path<CategoryIdPath>,
        Json(body): Json<CategoryByIdAction>,
    ) -> Result<Json<Category>> {
        tracing::debug!("act_collection_by_id {:?}", body);
        match body {
            CategoryByIdAction::Update(param) => {
                let res = ctrl.update(auth, id, param).await?;
                Ok(Json(res))
            }
            CategoryByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
        }
    }
    pub async fn get_category(
        State(ctrl): State<Self>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<CategoryParam>,
    ) -> Result<Json<CategoryGetResponse>> {
        tracing::debug!("get_collection {:?}", q);

        match q {
            CategoryParam::Query(param) => Ok(Json(CategoryGetResponse::Query(
                ctrl.query(auth, param).await?,
            ))),
        }
    }

    pub fn get_category_by_id(
        State(ctrl): State<Self>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(CategoryIdPath { id }): Path<CategoryIdPath>,
    ) -> Result<Json<Category>> {
        let user_id = match auth {
            Some(Authorization::Bearer { claims }) => claims
                .custom
                .get("id")
                .ok_or(ServiceError::Unauthorized)?
                .parse()
                .map_err(|e| {
                    tracing::error!("failed to parse id {e}");
                    ServiceError::Unauthorized
                })?,
            _ => {
                return Err(ServiceError::Unauthorized);
            }
        };

        Ok(Json(
            Category::query_builder(user_id)
                .id_equals(id)
                .query()
                .map(Category::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }
}
