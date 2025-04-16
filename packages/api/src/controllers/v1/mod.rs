use bdk::prelude::*;

use sqlx::{Pool, Postgres};
mod agit;
mod artist;
mod artwork;
mod collection;
mod users;
mod collector;

use agit::AgitControllerV1;
use artist::ArtistControllerV1;
use artwork::ArtworkControllerV1;
use users::UserController;

pub fn routes(pool: Pool<Postgres>) -> common::Result<by_axum::router::BiyardRouter> {
    Ok((by_axum::router::BiyardRouter::new())
        .nest("/agits", AgitControllerV1::route(pool.clone())?)
        .nest("/artworks", ArtworkControllerV1::route(pool.clone())?)
        .nest("/artists", ArtistControllerV1::route(pool.clone())?)
        .nest("/users", UserController::new(pool.clone()).route()?))
}
