use std::path::Path;
use regex::Regex;
use rocket::http::ContentType;
use crate::models::data::bank_record_report::BankRecordReport;
use crate::models::bank_accounts::{Bank, BankAccount};
use elan_financial_csv_converter::ElanFinancialCsvConverter;
use independent_bank_csv_converter::IndependentBankCsvConverter;
use crate::models::data::application_error::ApplicationError;
use crate::services::converters::american_express_credit_csv_converter::AmericanExpressCreditCsvConverter;
use crate::services::converters::venmo_csv_converter::VenmoCsvConverter;
use crate::services::converters::american_express_savings_csv_converter::AmericanExpressSavingsCsvConverter;
use crate::services::converters::huntington_csv_converter::HuntingtonCsvConverter;
use crate::services::factories::Factory;

pub mod independent_bank_csv_converter;
pub mod elan_financial_csv_converter;
pub mod venmo_csv_converter;
pub mod american_express_savings_csv_converter;
pub mod american_express_credit_csv_converter;
pub mod huntington_csv_converter;

mod converter_util;

pub trait BankRecordConverter<'a> {
    fn convert(&self, path: &Path, bank_account: &BankAccount) -> Result<BankRecordReport, ApplicationError>;
}

pub struct BankRecordConverterFactory;

impl<'a, 'b, 'c> Factory<(&'a Bank, &'b Path, &'c ContentType), Box<dyn BankRecordConverter<'a>>> for BankRecordConverterFactory {
    fn get_instance(key: (&'a Bank, &'b Path, &'c ContentType)) -> Option<Box<dyn BankRecordConverter<'a>>> {

        let bank_account = key.0;
        let path = key.1;
        let content_type = key.2;

        if path.file_name().is_none() {
            return None;
        }

        let filename = path.file_name().unwrap().to_str().unwrap();

        println!("filename: {:?}", filename);

        match bank_account {
            Bank::IndependentBank => {
                if content_type == &ContentType::CSV {
                    let filename_regex = Regex::new(r"export_\d{8}.*\.csv").unwrap();
                    if filename_regex.is_match(filename) {
                        return Some(Box::new(IndependentBankCsvConverter));
                    }
                }
                None
            }
            Bank::ElanFinancial => {
                if content_type == &ContentType::CSV {
                    let filename_regex = Regex::new(r"^Credit Card - \d{4}_\d{2}-\d{2}-\d{4}_\d{2}-\d{2}-\d{4}.*\.csv$").unwrap();
                    if filename_regex.is_match(filename) {
                        return Some(Box::new(ElanFinancialCsvConverter));
                    }
                }
                None
            }
            Bank::Venmo => {
                if content_type == &ContentType::CSV {
                    let filename_regex = Regex::new(r"^VenmoStatement_[A-Z][a-z]+_\d{4}.*\.csv$").unwrap();
                    if filename_regex.is_match(filename) {
                        return Some(Box::new(VenmoCsvConverter));
                    }
                }
                None
            }
            Bank::AmericanExpressSavings => {
                if content_type == &ContentType::CSV {
                    let filename_regex = Regex::new(r"^transactions.*\.CSV$").unwrap();
                    if filename_regex.is_match(filename) {
                        return Some(Box::new(AmericanExpressSavingsCsvConverter));
                    }
                }
                None
            }
            Bank::AmericanExpressCredit => {
                if content_type == &ContentType::CSV {
                    let filename_regex = Regex::new(r"^activity.*\.csv$").unwrap();
                    if filename_regex.is_match(filename) {
                        return Some(Box::new(AmericanExpressCreditCsvConverter));
                    }
                }
                None
            }
            Bank::Huntington => {
                if content_type == &ContentType::CSV {
                    let filename_regex = Regex::new(r"^Transactions_\d{2}-\d{2}-\d{4}.*\.csv$").unwrap();
                    if filename_regex.is_match(filename) {
                        return Some(Box::new(HuntingtonCsvConverter));
                    }
                }
                None
            }
        }
    }
}