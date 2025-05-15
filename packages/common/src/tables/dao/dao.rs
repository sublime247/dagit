use bdk::prelude::*;
#[api_model(base="/v1/dao", table = dao, action_by_id = [delete])]
pub struct Dao {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, action = create, type=JSON)]
    pub catergory: Vec<String>,
    #[api_model(summary, action = create, type=JSON, nullable)]
    pub general: Vec<String>,
    #[api_model(summary, action = create, type=JSONB, nullable)]
    pub governance: Vec<String>,
    #[api_model(summary, action = create, type=JSONB, nullable)]
    pub relations: Vec<String>,
    #[api_model(summary, action = create, type=JSONB, nullable)]
    pub exhibition_event: Vec<String>,
    #[api_model(summary, action = create, type=JSONB, nullable)]
    pub support: Vec<String>,
    #[api_model(summary, action = create, type=JSONB, nullable)]
    pub others: Vec<String>,
}
