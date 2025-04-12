use sqlx::{Type, FromRow};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BankAccount {
    pub id: Option<i64>,
    pub title: String,
    pub bank: Bank,
    pub account_type: AccountType,
    pub balance: i32
}

#[derive(Debug, Type, Serialize, Deserialize, PartialEq, Eq)]
#[repr(i32)]
pub enum Bank {
    IndependentBank = 1,
    ElanFinancial = 2,
    Venmo = 3,
    AmericanExpressSavings = 4,
    AmericanExpressCredit = 5,
    Huntington = 6,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[repr(i32)]
pub enum AccountType {
    Checking = 1,
    Savings = 2,
    Credit = 3,
    Investment = 4,
    Retirement = 5,
}