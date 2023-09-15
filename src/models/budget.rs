use chrono::NaiveDate;
use sqlx::{FromRow, Type};
use serde::{Serialize, Deserialize};
use crate::models::transaction_categories::CategoryClassification;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Budget {
    pub id: Option<i64>,
    pub title: String,
    pub classification: CategoryClassification,
    pub budget_type: BudgetType,
    pub user_id: i64,
    pub sort_order: i32,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BudgetHistory {
    pub id: Option<i64>,
    pub budget_id: i64,
    pub amount: i32,
    pub time_months: u32,
    pub payment_date: NaiveDate,
    pub recur_count: Option<u32>,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[repr(i32)]
pub enum BudgetType {
    Fixed = 1,
    Variable = 2,
}