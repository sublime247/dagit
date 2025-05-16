use crate::tables::categories::Category;
use bdk::prelude::*;
#[api_model(base="/v1/dao", table = dao, action_by_id = [delete])]
pub struct Dao {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, many_to_one = agits)]
    pub agit_id: i64,
    #[api_model(summary, action=create one_to_many=catergories, foreign_key = dao_id, type=JSON)]
    pub catergories: Vec<Category>,
}
