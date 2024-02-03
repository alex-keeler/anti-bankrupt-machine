use std::collections::HashMap;
use crate::models::data::bank_record_key::BankRecordKey;
use crate::models::bank_transactions::BankTransaction;

pub fn build_bank_record_transaction_map(transactions: Vec<BankTransaction>) -> HashMap<BankRecordKey, Vec<BankTransaction>> {

    let mut transaction_map = HashMap::new();

    for transaction in transactions {
        let transaction_key_list = transaction_map
            .entry(BankRecordKey::from_bank_transaction(&transaction).unwrap())
            .or_insert(Vec::new());

        transaction_key_list.push(transaction);
    }

    transaction_map
}