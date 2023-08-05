use sqlx::{Type, FromRow};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BankAccountUser {
    id: Option<i64>,
    pub user_id: i64,
    pub bank_account_id: i64,
    pub role: BankAccountUserRole
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[repr(i32)]
pub enum BankAccountUserRole {
    Owner = 1,
    Editor = 2,
    Viewer = 3
}