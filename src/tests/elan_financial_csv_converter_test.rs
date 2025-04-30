use chrono::NaiveDate;
use crate::models::bank_accounts::{AccountType, Bank, BankAccount};
use crate::models::bank_transactions::BankTransaction;
use crate::models::data::application_error::ApplicationError;
use crate::models::data::bank_record_report::BankRecordReport;
use crate::services::converters::BankRecordConverter;
use crate::services::converters::elan_financial_csv_converter::ElanFinancialCsvConverter;
use crate::tests::converter_test_util;

#[test]
pub fn test_converter() -> Result<(), ApplicationError> {

    let temp_file = converter_test_util::build_temp_file(vec![
        "\"Date\",\"Transaction\",\"Name\",\"Memo\",\"Amount\"",
        "\"2025-04-07\",\"DEBIT\",\"Test Description #1\",\"12345678901234567890123; 12345; ; ; ;\",\"-969.11\"",
        "\"2025-04-14\",\"DEBIT\",\"Test Description #2\",\"23456789012345678901234; 12345; ; ; ;\",\"-50.00\"",
        "\"2025-04-15\",\"CREDIT\",\"PAYMENT   THANK YOU\",\"WEB AUTOMTC; ; ; ; ;\",\"1024.68\"",
        "\"2025-04-20\",\"DEBIT\",\"ANNUAL MEMBERSHIP FEE\",\"; ; ; ; ;\",\"-0.00\"",
    ])?;

    let path = temp_file.path();
    
    let bank_account = BankAccount {
        id: Some(1),
        title: "Elan Financial - Credit".to_string(),
        bank: Bank::ElanFinancial,
        account_type: AccountType::Credit,
        balance: 0,
    };

    let converter = ElanFinancialCsvConverter;
    let report = converter.convert(&path, &bank_account)?;

    let expected_report = BankRecordReport {
        transactions: vec![
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2025, 04, 07).unwrap(),
                bank_description: Some(String::from("Test Description #1")),
                user_description: None,
                comment: None,
                amount: -96911,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2025, 04, 14).unwrap(),
                bank_description: Some(String::from("Test Description #2")),
                user_description: None,
                comment: None,
                amount: -5000,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2025, 04, 15).unwrap(),
                bank_description: Some(String::from("PAYMENT   THANK YOU")),
                user_description: None,
                comment: None,
                amount: 102468,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2025, 04, 20).unwrap(),
                bank_description: Some(String::from("ANNUAL MEMBERSHIP FEE")),
                user_description: None,
                comment: None,
                amount: -0,
                bank_account_id: 1,
            },
        ],
        begin_date: Some(NaiveDate::from_ymd_opt(2025, 04, 07).unwrap()),
        end_date: Some(NaiveDate::from_ymd_opt(2025, 04, 20).unwrap()),
    };


    assert_eq!(expected_report, report);

    Ok(())
}