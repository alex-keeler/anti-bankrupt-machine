use std::path::Path;
use sqlx::types::chrono::NaiveDate;

use crate::models::data::bank_record_report::BankRecordReport;
use crate::models::bank_accounts::BankAccount;
use crate::models::bank_transactions::BankTransaction;
use crate::models::data::application_error::ApplicationError;
use crate::services::converters::{converter_util, BankRecordConverter};

pub struct AmericanExpressCreditCsvConverter;

const EXPECTED_HEADERS: [&'static str; 5] = ["Date", "Description", "Card Member", "Account #", "Amount"];

const DATE_FIELD: usize = 0;
const BANK_DESCRIPTION_FIELD: usize = 1;
// const CARD_MEMBER_FIELD: usize = 2;
// const ACCOUNT_NUMBER_FIELD: usize = 3;
const AMOUNT_FIELD: usize = 4;

impl BankRecordConverter<'_> for AmericanExpressCreditCsvConverter {
    fn convert(&self, path: &Path, bank_account: &BankAccount) -> Result<BankRecordReport, ApplicationError> {
        let transactions = parse_data(path, bank_account)?;

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

fn parse_data(path: &Path, bank_account: &BankAccount) -> Result<Vec<BankTransaction>, ApplicationError> {
    let mut reader = csv::Reader::from_path(path).or(Err("Could not read file"))?;

    converter_util::verify_headers(reader.headers()?, Box::new(&EXPECTED_HEADERS))?;

    let mut transactions: Vec<BankTransaction> = Vec::new();
    for result in reader.records() {
        let record = result?;

        let date = NaiveDate::parse_from_str(record.get(DATE_FIELD).ok_or("could not get date field")?, "%m/%d/%Y").or(Err("could not parse date field"))?;
        let bank_description = record.get(BANK_DESCRIPTION_FIELD).ok_or("could not get bank field")?;
        let amount = parse_amount_from_str(record.get(AMOUNT_FIELD).ok_or("could not get amount field")?).or(Err("could not parse amount"))?;

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
    // the amount after the decimal might not be two digits so we have to parse the dollar and cent amount

    let amount = amount.replace("$", "")
        .replace(",", "")
        .replace(".", "")
        .trim()
        .parse::<i32>()? * -1;

    Ok(amount)
}