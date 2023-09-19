use sqlx::{Type, FromRow};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransactionCategory {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub category_group_id: i64,
    pub user_id: i64,
}

#[derive(Debug, FromRow, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransactionCategoryGroup {
    pub id: Option<i64>,
    pub title: String,
    pub classification: CategoryClassification,
}

#[derive(Debug, Type, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CategoryClassification {
    Needs = 1,
    Savings = 2,
    Wants = 3,
    Income = 4,
}

