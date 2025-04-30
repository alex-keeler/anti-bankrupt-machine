use std::path::Path;
use sqlx::types::chrono::NaiveDate;

use crate::models::data::bank_record_report::BankRecordReport;
use crate::models::bank_accounts::BankAccount;
use crate::models::bank_transactions::BankTransaction;
use crate::models::data::application_error::ApplicationError;
use crate::services::converters::{converter_util, BankRecordConverter};

pub struct VenmoCsvConverter;

const EXPECTED_HEADERS: [&'static str; 22] = [
    "", "ID", "Datetime", "Type", "Status", "Note", "From", "To", "Amount (total)", "Amount (tip)",
    "Amount (tax)", "Amount (fee)", "Tax Rate", "Tax Exempt", "Funding Source", "Destination",
    "Beginning Balance", "Ending Balance", "Statement Period Venmo Fees", "Terminal Location",
    "Year to Date Venmo Fees", "Disclaimer"
];

// const ID_FIELD: usize = 1;
const DATETIME_FIELD: usize = 2;
const PAYMENT_TYPE_FIELD: usize = 3;
// const STATUS_FIELD: usize = 4;
const TITLE_FIELD: usize = 5;
const FROM_FIELD: usize = 6;
const TO_FIELD: usize = 7;
const AMOUNT_FIELD: usize = 8;
// const AMOUNT_TIP_FIELD: usize = 9;
// const AMOUNT_TAX_FIELD: usize = 10;
// const AMOUNT_FEE_FIELD: usize = 11;
// const TAX_RATE_FIELD: usize = 12;
// const TAX_EXEMPT_FIELD: usize = 13;
// const FUNDING_SOURCE_FIELD: usize = 14;
// const DESTINATION_FIELD: usize = 15;
// const BEGINNING_BALANCE_FIELD: usize = 16;
// const ENDING_BALANCE_FIELD: usize = 17;
// const STATEMENT_PERIOD_VENMO_FEES_FIELD: usize = 18;
// const TERMINAL_LOCATION_FIELD: usize = 19;
// const YEAR_TO_DATE_VENMO_FEES_FIELD: usize = 20;
// const DISCLAIMER_FIELD: usize = 21;

impl BankRecordConverter<'_> for VenmoCsvConverter {
    fn convert(&self, path: &Path, bank_account: &BankAccount) -> Result<BankRecordReport, ApplicationError> {
        let transactions = parse_data(path, bank_account)?;

        if transactions.is_empty() {
            return Ok(BankRecordReport::empty());
        }

        // dates are kept in ascending order in this file
        let begin_date = transactions.get(0).unwrap().date;
        let end_date = transactions.get(transactions.len() - 1).unwrap().date;

        println!("begin: {:?}, end: {:?}", begin_date, end_date);

        Ok(BankRecordReport {
            transactions,
            begin_date: Some(begin_date),
            end_date: Some(end_date),
        })
    }
}


// TODO: proper error handling using results
fn parse_data(path: &Path, bank_account: &BankAccount) -> Result<Vec<BankTransaction>, ApplicationError> {
    let mut reader = csv::Reader::from_path(path)?;

    let mut transactions: Vec<BankTransaction> = Vec::new();

    let mut records = Vec::new();
    for record in reader.records() {
        records.push(record?);
    }

    converter_util::verify_headers(&records[1], Box::new(&EXPECTED_HEADERS))?;

    for record in &records[3..records.len() - 1] {
        let date = NaiveDate::parse_from_str(record.get(DATETIME_FIELD).ok_or("could not get date field")?, "%Y-%m-%dT%H:%M:%S").or(Err("could not parse date field"))?;
        let title = record.get(TITLE_FIELD).ok_or("could not get title field")?.trim();
        let payment_type = record.get(PAYMENT_TYPE_FIELD).ok_or("could not get payment type field")?.trim();
        let from = record.get(FROM_FIELD).ok_or("could not get from field")?.trim();
        let to = record.get(TO_FIELD).ok_or("could not get to field")?.trim();
        let amount = parse_amount_from_str(record.get(AMOUNT_FIELD).ok_or("could not get amount field")?)?;

        let comment = format!("{} from {} to {}", payment_type, from, to);

        let transaction = BankTransaction {
            id: None,
            date,
            bank_description: Some(title.to_string()),
            user_description: None,
            comment: Some(comment),
            amount,
            bank_account_id: bank_account.id.unwrap(),
        };

        transactions.push(transaction);
    }

    Ok(transactions)
}

fn parse_amount_from_str(amount: &str) -> Result<i32, ApplicationError> {
    let amount = amount.replace("$", "")
        .replace("+", "")
        .replace(" ", "")
        .replace(",", "")
        .replace(".", "")
        .trim()
        .parse()?;

    Ok(amount)
}