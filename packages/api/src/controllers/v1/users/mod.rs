#[cfg(test)]
mod tests;

use std::collections::HashMap;

use bdk::prelude::{by_axum::auth::generate_jwt, *};
use by_axum::{
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
        routing::post,
    },
};
use by_types::{Claims, JsonWithHeaders};
use common::Result;
use common::error::ServiceError;
use common::tables::prelude::*;
use sqlx::postgres::PgRow;

#[derive(Clone, Debug)]
pub struct UserController {
    pool: sqlx::PgPool,
    repo: UserRepository,
}

impl UserController {
    pub fn new(pool: sqlx::PgPool) -> Self {
        let repo = User::get_repository(pool.clone());
        Self { repo, pool }
    }
    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act).get(Self::get))
            .with_state(self.clone()))
    }
}

impl UserController {
    pub async fn act(
        State(ctrl): State<UserController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<UserAction>,
    ) -> Result<by_types::JsonWithHeaders<User>> {
        match body {
            UserAction::Signup(req) => ctrl.signup(req).await,
            UserAction::Login(req) => ctrl.login(req).await,
            UserAction::UpdateProfile(req) => ctrl.update_profile(req, auth).await,
        }
    }

    pub async fn get(
        State(ctrl): State<UserController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<UserParam>,
    ) -> Result<Json<User>> {
        tracing::debug!("act {:?}", q);

        match q {
            UserParam::Read(param)
                if param.action == Some(UserReadActionType::GetUserByAddress) =>
            {
                let user = ctrl.get_user_by_address(param).await?;
                Ok(Json(user))
            }
            _ => {
                todo!()
            }
        }
    }
}

impl UserController {
    fn generate_token(&self, user: &User) -> Result<String> {
        let mut claims = Claims {
            sub: user.address.clone(),
            exp: 0,
            role: by_types::Role::User,
            custom: HashMap::from([
                ("email".to_string(), user.email.to_string()),
                ("id".to_string(), user.id.to_string()),
            ]),
        };
        Ok(generate_jwt(&mut claims).map_err(|e| {
            tracing::error!("jwt generation error: {:?}", e);
            ServiceError::JwtGenerationFailed(e.to_string())
        })?)
    }

    async fn get_user_by_address(
        &self,
        UserReadAction { address, .. }: UserReadAction,
    ) -> Result<User> {
        let user: User = User::query_builder()
            .address_equals(address.unwrap_or_default())
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn signup(
        &self,
        UserSignupRequest {
            address,
            name,
            email,
            profile_url,
            provider,
        }: UserSignupRequest,
    ) -> Result<JsonWithHeaders<User>> {
        let user = self
            .repo
            .insert(provider, address, email, name, profile_url)
            .await?;
        let jwt = self.generate_token(&user)?;
        Ok(JsonWithHeaders::new(user).with_bearer_token(&jwt))
    }

    async fn login(
        &self,
        UserLoginRequest { address, provider }: UserLoginRequest,
    ) -> Result<JsonWithHeaders<User>> {
        let user = User::query_builder()
            .address_equals(address)
            .provider_equals(provider)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&self.pool)
            .await?;

        let jwt = self.generate_token(&user)?;
        Ok(JsonWithHeaders::new(user).with_bearer_token(&jwt))
    }

    pub async fn update_profile(
        &self,
        req: UserUpdateProfileRequest,
        auth: Option<Authorization>,
    ) -> Result<JsonWithHeaders<User>> {
        let principal = match auth {
            Some(Authorization::Bearer { claims }) => claims.sub,
            _ => {
                return Err(common::error::ServiceError::Unauthorized);
            }
        };

        let mut repo_req = UserRepositoryUpdateRequest::new().with_name(req.name);
        if req.profile_url.is_some() {
            repo_req = repo_req.with_profile_url(req.profile_url.unwrap());
        }

        let user = self
            .repo
            .find_one(&UserReadAction::new().get_user_by_address(principal))
            .await?;

        let user = self.repo.update(user.id, repo_req).await?;
        Ok(JsonWithHeaders::new(user))
    }
}
