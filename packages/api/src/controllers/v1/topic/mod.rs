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
    tables::topics::{
        Topic, TopicAction, TopicByIdAction, TopicCreateRequest, TopicGetResponse, TopicParam,
        TopicQuery, TopicRepository, TopicSummary, TopicUpdateRequest,
    },
};
use sqlx::postgres::PgRow;
#[cfg(test)]
mod tests;
#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct TopicPathParam{
    pub category_id: i64,
}
#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct TopicIdPath{
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct TopicIdPathParam{
    pub id: i64,
    pub category_id: i64,
}

#[derive(Clone, Debug)]
pub struct TopicController {
    pool:sqlx::PgPool,
    repo: TopicRepository,
}

impl TopicController{
    async fn query(
        &self,
        auth: Option<Authorization>,
        param: TopicQuery,
    ) -> Result<by_types::QueryResponse<TopicSummary>> {
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
        let items: Vec<TopicSummary> = TopicSummary::query_builder(user_id)
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
        category_id: i64,
        TopicCreateRequest { title, description }: TopicCreateRequest,
    )-> Result<Json<Topic>> {
        if auth.is_none() {
            return Err(ServiceError::Unauthorized);
        }
        let topic = self.repo.insert(category_id, title, description).await?;
        tracing::debug!("create topic: {:?}", topic);
        Ok(Json(topic))
    }
}






