use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct TransactionDetails {
    pub id: i64,
    pub date: NaiveDate,
    pub bank_description: Option<String>,
    pub user_description: Option<String>,
    pub comment: Option<String>,
    pub amount: i32,
    pub bank_account_title: String,
}