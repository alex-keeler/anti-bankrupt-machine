use chrono::NaiveDate;
use crate::models::bank_transactions::BankTransaction;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct BankRecordKey {
    pub date: NaiveDate,
    pub bank_description: String,
    pub amount: i32,
    pub bank_account_id: i64
}

impl BankRecordKey {
    pub fn from_bank_transaction(transaction: &BankTransaction) -> Option<Self> {
        if transaction.bank_description.is_none() {
            return None;
        }

        // TODO: figure out how to properly implement string slice here to avoid clone on bank description
        Some(BankRecordKey {
            date: transaction.date,
            bank_description: transaction.bank_description.clone().unwrap(),
            amount: transaction.amount,
            bank_account_id: transaction.bank_account_id,
        })
    }
}