#[derive(Clone, Debug, PartialEq)]
pub struct Collector {
    pub id: String,
    pub collector_id: String,
    pub total_volume: f64,
    pub total_volume_usd: f64,
    pub owned: i32,
    pub token_ids: Vec<String>,
    pub wallet_address: String,
    pub last_activity: String,
    pub verified: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)] // Replace Params with a valid derive macro
pub struct CollectorDetailParams {
    id: String,
}
