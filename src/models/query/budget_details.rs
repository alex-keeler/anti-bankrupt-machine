use chrono::NaiveDate;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use crate::models::budget::{BudgetClassification, SpendingType};

#[derive(Debug, FromRow, Serialize, Deserialize, PartialEq, Eq)]
pub struct BudgetDetails {
    pub id: i64,
    pub title: String,
    pub classification: BudgetClassification,
    pub spending_type: SpendingType,
    pub cumulative: bool,
    pub rollover: bool,
    pub start_date: NaiveDate,
    pub amount_monthly: i32,
    pub amount_cumulative: i32,
}