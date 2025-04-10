#[derive(Clone, Debug, PartialEq)]
pub struct Collector {
    pub id: String,
    pub collector_id: String,
    pub  total_volume: f64,
    pub  total_volume_usd: f64,
    pub  owned: i32,
    pub  token_ids: Vec<String>,
    pub  wallet_address: String,
    pub last_activity: String,
    pub  verified: bool,
}





#[derive(Clone, Debug, PartialEq)]
pub struct Asset {
    pub id: String,
    pub title: String,
    pub artist_name: String,
    pub attributes: Vec<String>,
    pub  way_to_sell: String,
    pub owner: String,
    pub current_price: f64,
    pub  current_price_usd: f64,
    pub average_price: f64,
    pub average_price_usd: f64,
    pub  price_change_24h: f64,
    pub   price_change_7d: f64,
    pub   volume: f64,
    pub  volume_usd: f64,
    pub  royalty: f64,
    pub   royalty_usd: f64,
    pub   status: String,
    pub   verified: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)] // Replace Params with a valid derive macro
pub struct CollectorDetailParams {
    id: String,
}