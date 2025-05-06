
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct FilterState {
    pub show_filters: bool,
    pub selected_currency: String,
    pub price_min: String,
    pub price_max: String,
    pub artist_search: String,
    pub attributes_search: String,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
#[allow(private_interfaces)]
#[derive(PartialEq)]
pub struct Artwork {
    pub id: usize,
    pub title: String,
    pub artist_name: String,
    pub verified: bool,
    pub collection: Option<String>,
    pub attributes: Vec<String>,
    pub ways_to_sell: String,
    pub volume_eth: f64,
    pub volume_usd: f64,
    pub status: String,
}
