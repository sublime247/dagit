use bdk::prelude::*;
use validator::Validate;

//TODO(api): Implement "/likes" and "/prices" API
//TODO(api): Implement admin api "m1/agit/:id" for manging admins.
//TODO(api): Implement Watermarking API
#[derive(Validate)]
#[api_model(base = "/v1/artworks", table = artworks, action_by_id = [delete], iter_type = by_types::QueryResponse)]
pub struct Artwork {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub name: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub description: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub verified: bool,
    #[api_model(summary, action = create, action_by_id = update)]
    pub collection_type: Option<String>,
    #[api_model(summary, action = create, action_by_id = update, type=JSONB)]
    pub attributes_type: Vec<String>,
    #[api_model(summary, action = create, action_by_id = update)]
    pub ways_to_sell: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub volume_eth: f64,
    #[api_model(summary, action = create, action_by_id = update)]
    pub volume_usd: f64,
    #[api_model(summary, action = create, action_by_id = update)]
    pub status: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub current_price: f64,
    #[api_model(summary, action = create, action_by_id = update)]
    pub average_price: f64,
    #[api_model(summary, action = create, action_by_id = update)]
    pub royalty: f64,
    #[api_model(summary, action = create, action_by_id = update)]
    pub price_change: f64,
    #[api_model(summary, action = create, action_by_id = update)]
    pub owners: i64,
    #[api_model(summary, action = create, action_by_id = update)]
    pub art_image: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub medium: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub rarity: String,

    #[api_model(summary, action = create, action_by_id = update)]
    pub activity_id: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub activity_from: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub activity_to: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub activity_time: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub activity_title: String,

    #[api_model(summary, action = create, action_by_id = update, nullable)]
    pub external_link: Option<String>,

    #[api_model(summary, many_to_one = agits)]
    pub agit_id: i64,

    #[api_model(summary, many_to_one = collections, nullable)]
    pub collection_id: Option<i64>,

    #[api_model(summary, many_to_one = artists)]
    pub artist_id: i64,

    #[api_model(summary, many_to_one = users)]
    pub owner_id: i64,

    #[api_model(summary, one_to_many = artwork_user_likes, foreign_key = artwork_id, aggregator = count)]
    pub likes: i64,
    #[api_model(summary, many_to_many = artwork_user_likes, table_name = users, foreign_primary_key = user_id, foreign_reference_key = artwork_id, aggregator = exist)]
    pub liked: bool,

    #[api_model(summary, one_to_many = artwork_prices, foreign_key = artwork_id, aggregator = max(created_at))]
    pub last_price: i64,
}
