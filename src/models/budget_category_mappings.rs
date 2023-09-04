use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BudgetCategoryMapping {
    pub id: Option<i32>,
    pub budget_id: i32,
    pub category_id: i32,
}