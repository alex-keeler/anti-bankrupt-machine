use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use crate::models::budget::BudgetType;
use crate::models::transaction_categories::CategoryClassification;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BudgetDetails {
    pub id: i64,
    pub title: String,
    pub classification: CategoryClassification,
    pub budget_type: BudgetType,
    pub amount: i32,
    pub time_months: u32,
    pub months_until_payment: u32,
    pub amount_monthly: i32,
    pub amount_spent: i32,
    pub amount_contributed: i32,
    pub amount_spent_month: i32,
    pub amount_contributed_month: i32,
}