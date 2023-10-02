use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::transaction_categories::CategoryClassification;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct TransactionDetails {
    pub id: i64,
    pub date: NaiveDate,
    pub bank_description: Option<String>,
    pub user_description: Option<String>,
    pub comment: Option<String>,
    pub amount: i32,
    pub category_id: Option<i64>,
    pub category_title: Option<String>,
    pub category_classification: Option<CategoryClassification>,
    pub contribution_budget_id: Option<i64>,
    pub contribution_budget_title: Option<String>,
    pub contribution_budget_classification: Option<CategoryClassification>,
    pub bank_account_title: String,
    pub hide_from_calculations: bool,
}