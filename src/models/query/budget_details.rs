use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use crate::models::budget::BudgetType;
use crate::models::transaction_categories::CategoryClassification;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BudgetDetails {
    pub id: i64,
    pub title: String,
    pub amount: i32,
    pub time_months: i32,
    pub on_payment_month: bool,
    pub budget_type: BudgetType,
    pub classification: CategoryClassification,
    pub amount_spent: i32,
}