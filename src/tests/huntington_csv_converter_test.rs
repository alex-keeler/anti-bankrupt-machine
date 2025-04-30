use chrono::NaiveDate;
use crate::models::bank_accounts::{AccountType, Bank, BankAccount};
use crate::models::bank_transactions::BankTransaction;
use crate::models::data::application_error::ApplicationError;
use crate::models::data::bank_record_report::BankRecordReport;
use crate::services::converters::BankRecordConverter;
use crate::services::converters::huntington_csv_converter::HuntingtonCsvConverter;
use crate::tests::converter_test_util;

#[test]
pub fn test_converter() -> Result<(), ApplicationError> {

    let temp_file = converter_test_util::build_temp_file(vec![
        "Date,Description,Category,Amount,Split,Tags",
        "1/1/25,Test Description #1,Clothing,-17.97,,",
        "1/2/25,Test Description #2,Groceries,-7.5,,",
        "1/2/25,Test Description #3,Fitness,-49.0,,",
        "1/3/25,Test Description #4,Other,1000.0,,",
    ])?;

    let path = temp_file.path();
    
    let bank_account = BankAccount {
        id: Some(1),
        title: "Huntington - Checking".to_string(),
        bank: Bank::Huntington,
        account_type: AccountType::Checking,
        balance: 0,
    };

    let converter = HuntingtonCsvConverter;
    let report = converter.convert(&path, &bank_account)?;

    let expected_report = BankRecordReport {
        transactions: vec![
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2025, 01, 01).unwrap(),
                bank_description: Some(String::from("Test Description #1")),
                user_description: None,
                comment: None,
                amount: -1797,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2025, 01, 02).unwrap(),
                bank_description: Some(String::from("Test Description #2")),
                user_description: None,
                comment: None,
                amount: -750,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2025, 01, 02).unwrap(),
                bank_description: Some(String::from("Test Description #3")),
                user_description: None,
                comment: None,
                amount: -4900,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2025, 01, 03).unwrap(),
                bank_description: Some(String::from("Test Description #4")),
                user_description: None,
                comment: None,
                amount: 100000,
                bank_account_id: 1,
            },
        ],
        begin_date: Some(NaiveDate::from_ymd_opt(2025, 01, 01).unwrap()),
        end_date: Some(NaiveDate::from_ymd_opt(2025, 01, 03).unwrap()),
    };


    assert_eq!(expected_report, report);

    Ok(())
}