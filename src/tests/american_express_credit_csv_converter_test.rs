use chrono::NaiveDate;
use crate::models::bank_accounts::{AccountType, Bank, BankAccount};
use crate::models::bank_transactions::BankTransaction;
use crate::models::data::application_error::ApplicationError;
use crate::models::data::bank_record_report::BankRecordReport;
use crate::services::converters::american_express_credit_csv_converter::AmericanExpressCreditCsvConverter;
use crate::services::converters::BankRecordConverter;
use crate::tests::converter_test_util;

#[test]
pub fn test_converter() -> Result<(), ApplicationError> {

    let temp_file = converter_test_util::build_temp_file(vec![
        "Date,Description,Card Member,Account #,Amount",
        "04/23/2025,Test Description #1,ALEXANDER S KEELER,-00000,2.79",
        "01/01/2025,Test Description #2,ALEXANDER S KEELER,-00000,-1024.68",
        "09/10/2024,Test Description #3,ALEXANDER S KEELER,-00000,48.60",
        "08/05/2024,Test Description #4,ALEXANDER S KEELER,-00000,23.00"
    ])?;

    let path = temp_file.path();
    
    let bank_account = BankAccount {
        id: Some(1),
        title: "American Express - Credit".to_string(),
        bank: Bank::AmericanExpressCredit,
        account_type: AccountType::Credit,
        balance: 0,
    };

    let converter = AmericanExpressCreditCsvConverter;
    let report = converter.convert(&path, &bank_account)?;

    let expected_report = BankRecordReport {
        transactions: vec![
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2025, 04, 23).unwrap(),
                bank_description: Some(String::from("Test Description #1")),
                user_description: None,
                comment: None,
                amount: -279,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2025, 01, 01).unwrap(),
                bank_description: Some(String::from("Test Description #2")),
                user_description: None,
                comment: None,
                amount: 102468,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2024, 09, 10).unwrap(),
                bank_description: Some(String::from("Test Description #3")),
                user_description: None,
                comment: None,
                amount: -4860,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2024, 08, 05).unwrap(),
                bank_description: Some(String::from("Test Description #4")),
                user_description: None,
                comment: None,
                amount: -2300,
                bank_account_id: 1,
            },
        ],
        begin_date: Some(NaiveDate::from_ymd_opt(2024, 08, 05).unwrap()),
        end_date: Some(NaiveDate::from_ymd_opt(2025, 04, 23).unwrap()),
    };


    assert_eq!(expected_report, report);

    Ok(())
}