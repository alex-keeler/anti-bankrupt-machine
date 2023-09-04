use chrono::NaiveDate;
use sqlx::{FromRow, Type};
use serde::{Serialize, Deserialize};
use crate::models::transaction_categories::CategoryClassification;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Budget {
    pub id: Option<i64>,
    pub title: String,
    pub amount: i32,
    pub time_months: i32,
    pub start_date: NaiveDate,
    pub budget_type: BudgetType,
    pub classification: CategoryClassification,
    // TODO: add user_id field, filter on it for repository queries
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[repr(i32)]
pub enum BudgetType {
    Fixed = 1,
    Variable = 2,
}