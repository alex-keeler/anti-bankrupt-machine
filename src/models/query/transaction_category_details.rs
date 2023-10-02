use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct TransactionCategoryDetails {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub category_group_id: i64,
    pub category_group_title: String,
    pub total_amount: i32,
}