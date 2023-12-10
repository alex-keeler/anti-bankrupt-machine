use chrono::NaiveDate;

use crate::models::bank_transactions::BankTransaction;

pub struct BankRecordReport {
    pub transactions: Vec<BankTransaction>,
    pub begin_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

impl BankRecordReport {
    pub fn empty() -> Self {
        BankRecordReport {
            transactions: Vec::new(),
            begin_date: None,
            end_date: None,
        }
    }
}