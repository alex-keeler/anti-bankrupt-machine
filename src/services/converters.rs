use std::error::Error;
use rocket::fs::TempFile;
use rocket::http::ContentType;
use crate::data::bank_record_report::BankRecordReport;
use crate::models::bank_accounts::{Bank, BankAccount};
use crate::services::converters::independent_bank_csv_converter::IndependentBankCsvConverter;
use crate::services::factories::{Factory, FactoryHandler};

pub mod independent_bank_csv_converter;

pub trait BankRecordConverter<'a>: FactoryHandler<(&'a Bank, &'a TempFile<'a>)> {
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
        }
    }
}