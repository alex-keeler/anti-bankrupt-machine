use serde::{Serialize, Deserialize};
use crate::models::transaction_categories::CategoryClassification;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionCategoryGroupSelect {
    pub id: i64,
    pub title: String,
    pub classification: CategoryClassification,
    pub categories: Vec<TransactionCategorySelect>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionCategorySelect {
    pub id: i64,
    pub title: Option<String>,
}