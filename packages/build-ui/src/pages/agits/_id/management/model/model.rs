#[derive(Clone, Debug, PartialEq)]
pub struct Assets {
    pub id: String,
    pub title: String,
    pub artist_name: String,
    pub attributes: Vec<String>,
    pub way_to_sell: String,
    pub owner: String,
    pub current_price: f64,
    pub current_price_usd: f64,
    pub average_price: f64,
    pub average_price_usd: f64,
    pub price_change_24h: f64,
    pub price_change_7d: f64,
    pub volume: f64,
    pub volume_usd: f64,
    pub royalty: f64,
    pub royalty_usd: f64,
    pub status: String,
    pub verified: bool,
    pub art_image: String,
    pub medium: String,
    pub rarity: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Activity{
    pub id: String,
    pub from: String,
    pub to: String,
    pub title: String,
    pub time: String,
}