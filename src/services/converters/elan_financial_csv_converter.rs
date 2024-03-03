use std::error::Error;

use rocket::fs::TempFile;
use sqlx::types::chrono::NaiveDate;

use crate::models::data::bank_record_report::BankRecordReport;
use crate::models::bank_accounts::BankAccount;
use crate::models::bank_transactions::BankTransaction;
use crate::services::converters::BankRecordConverter;

pub struct ElanFinancialCsvConverter;

const DATE_FIELD: usize = 0;
// const TRANSACTION_TYPE_FIELD = 1;
const BANK_DESCRIPTION_FIELD: usize = 2;
// const MEMO_FIELD: usize = 3;
const AMOUNT_FIELD: usize = 4;

impl BankRecordConverter<'_> for ElanFinancialCsvConverter {
    fn convert(&self, file: &TempFile<'_>, bank_account: &BankAccount) -> Result<BankRecordReport, Box<dyn Error>> {
        let transactions = parse_data(file, bank_account)?;

        if transactions.is_empty() {
            return Ok(BankRecordReport::empty());
        }

        // dates are kept in descending order in this file
        let begin_date = transactions.get(transactions.len() - 1).unwrap().date;
        let end_date = transactions.get(0).unwrap().date;

        Ok(BankRecordReport {
            transactions,
            begin_date: Some(begin_date),
            end_date: Some(end_date),
        })
    }
}


// TODO: proper error handling using results
fn parse_data(file: &TempFile<'_>, bank_account: &BankAccount) -> Result<Vec<BankTransaction>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(file.path().unwrap()).unwrap();

    let mut transactions: Vec<BankTransaction> = Vec::new();

    for result in reader.records() {
        let record = result?;

        let date = NaiveDate::parse_from_str(record.get(DATE_FIELD).ok_or("could not get date field").unwrap(), "%Y-%m-%d").unwrap();
        let bank_description = record.get(BANK_DESCRIPTION_FIELD).ok_or("could not get bank field").unwrap();
        let amount = parse_amount_from_str(record.get(AMOUNT_FIELD).ok_or("could not get amount field").unwrap()).unwrap();

        let transaction = BankTransaction {
            id: None,
            date,
            bank_description: Some(bank_description.to_string()),
            user_description: None,
            comment: None,
            amount,
            bank_account_id: bank_account.id.unwrap(),
        };

        println!("{:?}", record);
        transactions.push(transaction);
    }

    Ok(transactions)
}

fn parse_amount_from_str(amount: &str) -> Result<i32, Box<dyn Error>> {

    let amount = normalize_decimal(amount)
        .replace(",", "")
        .replace(".", "")
        .trim()
        .parse()?;

    Ok(amount)
}

fn normalize_decimal(amount: &str) -> String {

    let decimal_index = amount.find(".");

    if decimal_index.is_none() {
        return format!("{}.00", amount);
    }

    let decimal_index = decimal_index.unwrap();
    let decimal_digits = amount.len() - decimal_index - 1;

    if decimal_digits < 2 {
        return format!("{}.00", &amount[0..decimal_index]);
    }

    if decimal_digits > 2 {
        let digits_over = decimal_digits - 2;
        return String::from(&amount[0..(amount.len() - digits_over)]);
    }

    return String::from(amount);
}