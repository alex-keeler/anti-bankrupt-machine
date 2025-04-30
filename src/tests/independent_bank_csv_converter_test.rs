use chrono::NaiveDate;
use crate::models::bank_accounts::{AccountType, Bank, BankAccount};
use crate::models::bank_transactions::BankTransaction;
use crate::models::data::application_error::ApplicationError;
use crate::models::data::bank_record_report::BankRecordReport;
use crate::services::converters::american_express_credit_csv_converter::AmericanExpressCreditCsvConverter;
use crate::services::converters::BankRecordConverter;
use crate::services::converters::independent_bank_csv_converter::IndependentBankCsvConverter;
use crate::tests::converter_test_util;

#[test]
pub fn test_converter() -> Result<(), ApplicationError> {

    let temp_file = converter_test_util::build_temp_file(vec![
        "\"Date\",\"Description\",\"Comments\",\"Check Number\",\"Amount\",\"Balance\"",
        "\"04/23/2025\",\"Test Description #1\",\"\",\"\",\"-$25.20\",\"$1,299.48\"",
        "\"01/01/2025\",\"Test Description #2\",\"\",\"\",\"$300.00\",\"$1,324.68\"",
        "\"09/10/2024\",\"Test Description #3\",\"\",\"\",\"$1,024.68\",\"$1,024.68\"",
    ])?;

    let path = temp_file.path();
    
    let bank_account = BankAccount {
        id: Some(1),
        title: "Independent Bank - Checking".to_string(),
        bank: Bank::IndependentBank,
        account_type: AccountType::Checking,
        balance: 0,
    };

    let converter = IndependentBankCsvConverter;
    let report = converter.convert(&path, &bank_account)?;

    let expected_report = BankRecordReport {
        transactions: vec![
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2025, 04, 23).unwrap(),
                bank_description: Some(String::from("Test Description #1")),
                user_description: None,
                comment: None,
                amount: -2520,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2025, 01, 01).unwrap(),
                bank_description: Some(String::from("Test Description #2")),
                user_description: None,
                comment: None,
                amount: 30000,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2024, 09, 10).unwrap(),
                bank_description: Some(String::from("Test Description #3")),
                user_description: None,
                comment: None,
                amount: 102468,
                bank_account_id: 1,
            },
        ],
        begin_date: Some(NaiveDate::from_ymd_opt(2024, 09, 10).unwrap()),
        end_date: Some(NaiveDate::from_ymd_opt(2025, 04, 23).unwrap()),
    };


    assert_eq!(expected_report, report);

    Ok(())
}