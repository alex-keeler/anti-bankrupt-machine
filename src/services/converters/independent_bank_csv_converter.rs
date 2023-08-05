use std::collections::HashMap;
use std::error::Error;
use rocket::fs::TempFile;
use rocket::http::ContentType;
use crate::models::bank_accounts::{Bank, BankAccount};
use crate::models::bank_transactions::BankTransaction;
use crate::services::converters::BankRecordConverter;
use crate::services::factories::FactoryHandler;
use sqlx::types::chrono::NaiveDate;
use crate::data::bank_record_key::BankRecordKey;
use crate::data::bank_record_report::BankRecordReport;
use crate::util::bank_record_util;

pub struct IndependentBankCsvConverter;

const DATE_FIELD: usize = 0;
const BANK_DESCRIPTION_FIELD: usize = 1;
// const COMMENT_FIELD: usize = 2;
// const CHECK_NUMBER_FIELD: usize = 3;
const AMOUNT_FIELD: usize = 4;
// const BALANCE_FIELD: usize = 5;

fn parse_amount_from_str(amount: &str) -> Result<i32, Box<dyn Error>> {

    let amount = amount.replace("$", "")
                       .replace(",", "")
                       .replace(".", "")
                       .trim()
                       .parse().unwrap();

    Ok(amount)
}

impl FactoryHandler<(&'_ Bank, &'_ TempFile<'_>)> for IndependentBankCsvConverter {
    fn supports(&self, arg: &(&Bank, &TempFile<'_>)) -> bool {
        Bank::IndependentBank == *arg.0 &&
            arg.1.content_type().is_some() &&
            ContentType::CSV.eq(arg.1.content_type().unwrap())
    }
}

impl BankRecordConverter<'_> for IndependentBankCsvConverter {
    fn convert(&self, file: &TempFile<'_>, bank_account: &BankAccount) -> Result<BankRecordReport, Box<dyn Error>> {

        let transactions = self.parse_data(file, bank_account)?;

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

impl IndependentBankCsvConverter {
    fn parse_data(&self, file: &TempFile<'_>, bank_account: &BankAccount) -> Result<Vec<BankTransaction>, Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(file.path().unwrap()).unwrap();

        let mut transactions: Vec<BankTransaction> = Vec::new();
        for result in reader.records() {
            let record = result?;

            let date = NaiveDate::parse_from_str(record.get(DATE_FIELD).ok_or("could not get date field").unwrap(), "%m/%d/%Y").unwrap();
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
                category_id: None,
                saving_goal_id: None,
            };

            println!("{:?}", record);
            transactions.push(transaction);
        }

        Ok(transactions)
    }
}