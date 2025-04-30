use chrono::NaiveDate;
use crate::models::bank_accounts::{AccountType, Bank, BankAccount};
use crate::models::bank_transactions::BankTransaction;
use crate::models::data::application_error::ApplicationError;
use crate::models::data::bank_record_report::BankRecordReport;
use crate::services::converters::american_express_savings_csv_converter::AmericanExpressSavingsCsvConverter;
use crate::services::converters::BankRecordConverter;
use crate::services::converters::venmo_csv_converter::VenmoCsvConverter;
use crate::tests::converter_test_util;

#[test]
pub fn test_converter() -> Result<(), ApplicationError> {

    let temp_file = converter_test_util::build_temp_file(vec![
        "Account Statement - (@myvenmo) ,,,,,,,,,,,,,,,,,,,,,",
        "Account Activity,,,,,,,,,,,,,,,,,,,,,",
        ",ID,Datetime,Type,Status,Note,From,To,Amount (total),Amount (tip),Amount (tax),Amount (fee),Tax Rate,Tax Exempt,Funding Source,Destination,Beginning Balance,Ending Balance,Statement Period Venmo Fees,Terminal Location,Year to Date Venmo Fees,Disclaimer",
        ",,,,,,,,,,,,,,,,$0.00,,,,,",
        ",1234567890123456789,2024-12-03T22:13:03,Payment,Complete,Incoming Transaction #1,Sender 1,Alex Keeler,+ $5.00,,0,,0,,,Venmo balance,,,,Venmo,,",
        ",2345678901234567890,2024-12-06T04:21:43,Payment,Complete,Incoming Transaction #2,Sender 2,Alex Keeler,\"+ $1,250.00\",,0,,0,,,Venmo balance,,,,Venmo,,",
        ",3456789012345678901,2024-12-23T00:18:19,Merchant Transaction,Complete,,Alex Keeler,Merchant 1,- $14.37,,,,,,Venmo balance,,,,,Venmo,,",
        ",4567890123456789012,2024-12-29T20:30:24,Payment,Complete,Outgoing Transaction #1,Alex Keeler,Receiver 1,- $25.00,,0,,0,,Bank Account *0000,,,,,Venmo,,",
        ",5678901234567890123,2025-01-03T18:34:09,Payment,Complete,Outgoing Transaction #2,Alex Keeler,Receiver 2,- $20.00,,0,,0,,Venmo balance,,,,,Venmo,,",
        ",,,,,,,,,,,,,,,,,$15.00,$0.00,,$0.00,\"In case of errors or questions about your",
        "        electronic transfers:",
        "        - Telephone us at 855-812-4430",
        "        - Write the Venmo Error Resolution Department at",
        "          222  W. Merchandise Plaza, Suite 800, Chicago, IL 60654; or",
        "        - Write to us through the Contact Us page",
        "          (https://help.venmo.com/hc/en-us/requests/new)",
        "        Contact us as soon as you can if you think your statement or",
        "        receipt is wrong or if you need more information about",
        "        a transfer on the statement or receipt. We must hear from",
        "        you no later than 60 days after we sent you the FIRST",
        "        statement on which the error or problem appeared.",
        "        1. Tell us your name and username or phone number.",
        "        2. Describe the error or the transfer you are unsure about,",
        "           and explain as clearly as you can why you believe it is",
        "           an error or why you need more information.",
        "        3. Tell us the dollar amount of the suspected error.",
        "        We will investigate your complaint and will correct any",
        "        error promptly. If we take more than 10 business days to do",
        "        this, we will credit your account for the amount you think",
        "        is in error, so that you will have the use of the money",
        "        during the time it takes us to complete our investigation.",
        "        In case of errors or questions about your transactions made",
        "        with your Venmo Mastercard, please consult your",
        "        Cardholder Agreement",
        "        (https://venmo.com/legal/bancorp-cardholder-agreement).",
        "        \"",
        ""
    ])?;

    let path = temp_file.path();
    
    let bank_account = BankAccount {
        id: Some(1),
        title: "Venmo".to_string(),
        bank: Bank::Venmo,
        account_type: AccountType::Checking,
        balance: 0,
    };

    let converter = VenmoCsvConverter;
    let report = converter.convert(&path, &bank_account)?;

    let expected_report = BankRecordReport {
        transactions: vec![
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2024, 12, 03).unwrap(),
                bank_description: Some(String::from("Incoming Transaction #1")),
                user_description: None,
                comment: Some(String::from("Payment from Sender 1 to Alex Keeler")),
                amount: 500,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2024, 12, 06).unwrap(),
                bank_description: Some(String::from("Incoming Transaction #2")),
                user_description: None,
                comment: Some(String::from("Payment from Sender 2 to Alex Keeler")),
                amount: 125000,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2024, 12, 23).unwrap(),
                bank_description: Some(String::from("")),
                user_description: None,
                comment: Some(String::from("Merchant Transaction from Alex Keeler to Merchant 1")),
                amount: -1437,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2024, 12, 29).unwrap(),
                bank_description: Some(String::from("Outgoing Transaction #1")),
                user_description: None,
                comment: Some(String::from("Payment from Alex Keeler to Receiver 1")),
                amount: -2500,
                bank_account_id: 1,
            },
            BankTransaction {
                id: None,
                date: NaiveDate::from_ymd_opt(2025, 01, 03).unwrap(),
                bank_description: Some(String::from("Outgoing Transaction #2")),
                user_description: None,
                comment: Some(String::from("Payment from Alex Keeler to Receiver 2")),
                amount: -2000,
                bank_account_id: 1,
            },
        ],
        begin_date: Some(NaiveDate::from_ymd_opt(2024, 12, 03).unwrap()),
        end_date: Some(NaiveDate::from_ymd_opt(2025, 01, 03).unwrap()),
    };


    assert_eq!(expected_report, report);

    Ok(())
}