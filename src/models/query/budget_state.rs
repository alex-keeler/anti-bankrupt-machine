use chrono::NaiveDate;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use crate::models::budget::{BudgetClassification, SpendingType};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BudgetState {
    pub id: i64,
    pub title: String,
    pub classification: BudgetClassification,
    pub spending_type: SpendingType,
    pub cycle_start_date: NaiveDate,
    pub months_until_maturity: Option<u32>,
    pub amount_monthly: i32,
    pub amount_current: i32,
    pub amount_total: Option<i32>,
    pub amount_spent: i32,
    pub amount_spent_this_month: i32,
    pub amount_contributed: i32,
    pub amount_contributed_this_month: i32,
}