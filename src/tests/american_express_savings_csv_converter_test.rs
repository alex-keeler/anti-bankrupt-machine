use chrono::NaiveDate;
use crate::models::bank_accounts::{AccountType, Bank, BankAccount};
use crate::models::bank_transactions::BankTransaction;
use crate::models::data::application_error::ApplicationError;
use crate::models::data::bank_record_report::BankRecordReport;
use crate::services::converters::american_express_savings_csv_converter::AmericanExpressSavingsCsvConverter;
use crate::services::converters::BankRecordConverter;
use crate::tests::converter_test_util;

#[test]
pub fn test_converter() -> Result<(), ApplicationError> {

    let temp_file = converter_test_util::build_temp_file(vec![
        "2025-02-12,\"Test Description #1\",46.96",
        "2025-01-01,\"Test Description #2\",3000.0",
        "2024-09-11,\"Test Description #3\",-300.0",
        "2024-09-10,\"Test Description #4\",-4.6",
    ])?;

    let path = temp_file.path();
    
    let bank_account = BankAccount {
        id: Some(1),
        title: "American Express - Savings".to_string(),
        bank: Bank::AmericanExpressSavings,
        account_type: AccountType::Savings,
        balance: 0,
    };

    let converter = AmericanExpressSavingsCsvConverter;
    let report = converter.convert(&path, &bank_account)?;

    let expected_report = BankRecordReport {
        transactions: vec![
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2025, 02, 12).unwrap(),
                bank_description: Some(String::from("Test Description #1")),
                user_description: None,
                comment: None,
                amount: 4696,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2025, 01, 01).unwrap(),
                bank_description: Some(String::from("Test Description #2")),
                user_description: None,
                comment: None,
                amount: 300000,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2024, 09, 11).unwrap(),
                bank_description: Some(String::from("Test Description #3")),
                user_description: None,
                comment: None,
                amount: -30000,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2024, 09, 10).unwrap(),
                bank_description: Some(String::from("Test Description #4")),
                user_description: None,
                comment: None,
                amount: -460,
                bank_account_id: 1,
            },
        ],
        begin_date: Some(NaiveDate::from_ymd_opt(2024, 09, 10).unwrap()),
        end_date: Some(NaiveDate::from_ymd_opt(2025, 02, 12).unwrap()),
    };


    assert_eq!(expected_report, report);

    Ok(())
}