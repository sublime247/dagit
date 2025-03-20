use bdk::prelude::*;

#[api_model(base = "/v1/artists", table = artists, action_by_id = [delete], iter_type = by_types::QueryResponse)]
pub struct Artist {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,
}
