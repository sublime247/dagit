use bdk::prelude::*;

#[api_model(base="/v1/category", table = category, action_by_id = [delete])]
pub struct Category {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, action = create, action_by_id = update, type=JSON, nullable)]
    pub general: Vec<String>,
    #[api_model(summary, action = create, action_by_id = update, type=JSONB, nullable)]
    pub governance: Vec<String>,
    #[api_model(summary, action = create, action_by_id = update, type=JSONB, nullable)]
    pub relations: Vec<String>,
    #[api_model(summary, action = create, action_by_id = update, type=JSONB, nullable)]
    pub exhibition_event: Vec<String>,
    #[api_model(summary, action = create, action_by_id = update, type=JSONB, nullable)]
    pub support: Vec<String>,
    #[api_model(summary, action = create, action_by_id = update, type=JSONB, nullable)]
    pub others: Vec<String>,
}
