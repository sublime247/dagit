#[derive(Clone, Debug, PartialEq)]

pub struct Artist {
    pub id: String,
    pub name: String,
    pub mail: String,
    pub revenue: f64,
    pub revenue_usd: f64,
    pub attributes: Vec<String>,
    pub status: String,
    pub social_media: String,
    pub featured_work: String,
    pub artwork:String,


}
