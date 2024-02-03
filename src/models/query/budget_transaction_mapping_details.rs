use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use crate::models::budget_transaction_mapping::TransactionBehavior;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BudgetTransactionMappingDetails {
    pub id: i64,
    pub bank_transaction_id: i64,
    pub budget_id: i64,
    pub budget_title: String,
    pub amount: i32,
    pub behavior: TransactionBehavior,
}