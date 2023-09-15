use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use sqlx::types::chrono::NaiveDate;

#[derive(Debug, FromRow, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BankTransaction {
    pub id: Option<i64>,
    pub date: NaiveDate,
    pub bank_description: Option<String>,
    pub user_description: Option<String>,
    pub comment: Option<String>,
    pub amount: i32,
    pub bank_account_id: i64,
    pub category_id: Option<i64>,
    pub contribution_budget_id: Option<i64>
}