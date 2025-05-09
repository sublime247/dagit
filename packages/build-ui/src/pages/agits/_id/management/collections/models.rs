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
