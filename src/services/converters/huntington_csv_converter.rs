use std::path::Path;
use sqlx::types::chrono::NaiveDate;

use crate::models::data::bank_record_report::BankRecordReport;
use crate::models::bank_accounts::BankAccount;
use crate::models::bank_transactions::BankTransaction;
use crate::models::data::application_error::ApplicationError;
use crate::services::converters::{converter_util, BankRecordConverter};

pub struct HuntingtonCsvConverter;

const EXPECTED_HEADERS: [&'static str; 6] = ["Date", "Description", "Category", "Amount", "Split", "Tags"];

const DATE_FIELD: usize = 0;
const BANK_DESCRIPTION_FIELD: usize = 1;
// const CATEGORY_FIELD: usize = 2;
const AMOUNT_FIELD: usize = 3;
// const SPLIT_FIELD: usize = 4;
// const TAGS_FIELD: usize = 5;

impl BankRecordConverter<'_> for HuntingtonCsvConverter {
    fn convert(&self, path: &Path, bank_account: &BankAccount) -> Result<BankRecordReport, ApplicationError> {
        let transactions = parse_data(path, bank_account)?;

        if transactions.is_empty() {
            return Ok(BankRecordReport::empty());
        }

        // dates are kept in ascending order in this file
        let begin_date = transactions.get(0).unwrap().date;
        let end_date = transactions.get(transactions.len() - 1).unwrap().date;

        Ok(BankRecordReport {
            transactions,
            begin_date: Some(begin_date),
            end_date: Some(end_date),
        })
    }
}

fn parse_data(path: &Path, bank_account: &BankAccount) -> Result<Vec<BankTransaction>, ApplicationError> {
    let mut reader = csv::Reader::from_path(path)?;

    converter_util::verify_headers(reader.headers()?, Box::new(&EXPECTED_HEADERS))?;

    let mut transactions: Vec<BankTransaction> = Vec::new();
    for result in reader.records() {
        let record = result?;

        let date = NaiveDate::parse_from_str(record.get(DATE_FIELD).ok_or("could not get date field")?, "%m/%d/%y").or(Err("could not get date field"))?;
        let bank_description = record.get(BANK_DESCRIPTION_FIELD).ok_or("could not get bank field")?;
        let amount = parse_amount_from_str(record.get(AMOUNT_FIELD).ok_or("could not get amount field")?)?;

        let transaction = BankTransaction {
            id: None,
            date,
            bank_description: Some(bank_description.to_string()),
            user_description: None,
            comment: None,
            amount,
            bank_account_id: bank_account.id.unwrap(),
        };

        transactions.push(transaction);
    }

    Ok(transactions)
}

fn parse_amount_from_str(amount: &str) -> Result<i32, ApplicationError> {
    let decimal_index = amount.find('.');
    let digits_after_decimal = if decimal_index.is_some() { amount.len() - decimal_index.unwrap() - 1 } else { 0 };

    let amount = amount.replace(".", "")
        .trim()
        .parse::<i32>()? * 10_i32.pow(2 - digits_after_decimal as u32);

    Ok(amount)
}