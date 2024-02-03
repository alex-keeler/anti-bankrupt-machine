use chrono::NaiveDate;
use sqlx::{FromRow, Type};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Budget {
    pub id: Option<i64>,
    pub title: String,
    pub classification: BudgetClassification,
    pub spending_type: SpendingType,
    pub user_id: i64,
    pub sort_order: i32,
    pub cumulative: bool,
    pub hidden: bool,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BudgetHistory {
    pub id: Option<i64>,
    pub budget_id: i64,
    pub start_date: NaiveDate,
    pub amount_monthly: i32,
    pub months_until_maturity: Option<u32>,
    pub months_until_completion: Option<u32>,
    pub recur_count: Option<u32>,
}

#[derive(Debug, Type, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
#[repr(i32)]
pub enum BudgetClassification {
    Needs = 1,
    Savings = 2,
    Wants = 3,
    Income = 4,
    Internal = 5,
}

#[derive(Debug, Type, Serialize, Deserialize, PartialEq, Eq)]
#[repr(i32)]
pub enum SpendingType {
    Fixed = 1,
    Variable = 2,
}