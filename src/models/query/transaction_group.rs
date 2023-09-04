use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct TransactionGroup {
    pub bank_description: Option<String>,
    pub total_amount: i32,
    pub count: i32,
}