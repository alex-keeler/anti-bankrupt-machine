use rocket::form::FromFormField;
use sqlx::{FromRow, Type};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BudgetTransactionMapping {
    pub id: Option<i64>,
    pub budget_id: i64,
    pub bank_transaction_id: i64,
    pub amount: i32,
    pub behavior: TransactionBehavior,
}

#[derive(Debug, Type, Serialize, Deserialize, PartialEq, Eq, Hash, FromFormField)]
#[repr(i32)]
pub enum TransactionBehavior {
    Spending = 1,
    Contribution = 2,
    Reimbursement = 3,
}