use std::error::Error;
use rocket::fs::TempFile;
use rocket::http::ContentType;
use crate::models::data::bank_record_report::BankRecordReport;
use crate::models::bank_accounts::{Bank, BankAccount};
use elan_financial_csv_converter::ElanFinancialCsvConverter;
use independent_bank_csv_converter::IndependentBankCsvConverter;
use crate::services::converters::venmo_csv_converter::VenmoCsvConverter;
use crate::services::converters::american_express_csv_converter::AmericanExpressCsvConverter;
use crate::services::factories::Factory;

pub mod independent_bank_csv_converter;
pub mod elan_financial_csv_converter;
pub mod venmo_csv_converter;
pub mod american_express_csv_converter;

pub trait BankRecordConverter<'a> {
    fn convert(&self, file: &TempFile<'_>, bank_account: &BankAccount) -> Result<BankRecordReport, Box<dyn Error>>;
}

pub struct BankRecordConverterFactory;

impl<'a, 'b> Factory<(&'a Bank, &'b TempFile<'_>), Box<dyn BankRecordConverter<'a>>> for BankRecordConverterFactory {
    fn get_instance(key: (&'a Bank, &'b TempFile<'b>)) -> Option<Box<dyn BankRecordConverter<'a>>> {

        if key.1.content_type().is_none() {
            return None;
        }
        let content_type = key.1.content_type().unwrap();

        match key.0 {
            Bank::IndependentBank => {
                if content_type == &ContentType::CSV {
                    return Some(Box::new(IndependentBankCsvConverter));
                }
                None
            }
            Bank::ElanFinancial => {
                if content_type == &ContentType::CSV {
                    return Some(Box::new(ElanFinancialCsvConverter));
                }
                None
            }
            Bank::Venmo => {
                if content_type == &ContentType::CSV {
                    return Some(Box::new(VenmoCsvConverter));
                }
                None
            }
            Bank::AmericanExpress => {
                if content_type == &ContentType::CSV {
                    return Some(Box::new(AmericanExpressCsvConverter));
                }
                None
            }
        }
    }
}