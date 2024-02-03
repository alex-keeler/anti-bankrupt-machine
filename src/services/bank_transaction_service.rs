use std::error::Error;

use chrono::NaiveDate;
use rocket::fs::TempFile;
use rocket_db_pools::Pool;
use sqlx::{Acquire, MySql};
use sqlx::pool::PoolConnection;
use crate::models::data::application_error::ApplicationError;

use crate::models::data::bank_record_report::BankRecordReport;
use crate::models::bank_accounts::BankAccount;
use crate::models::bank_transactions::BankTransaction;
use crate::models::budget_transaction_mapping::TransactionBehavior;
use crate::models::data::full_transaction_details::FullTransactionDetails;
use crate::models::query::budget_transaction_mapping_details::BudgetTransactionMappingDetails;
use crate::models::query::transaction_details::TransactionDetails;
use crate::models::query::transaction_group::TransactionGroup;
use crate::models::users::User;
use crate::repositories::{bank_transaction_repository, budget_repository, budget_transaction_mapping_repository};
use crate::services::converters::BankRecordConverterFactory;
use crate::services::factories::Factory;
use crate::util::{bank_record_util, date_util};

pub async fn import_bank_record_file(
    file: &TempFile<'_>, _user: User, bank_account: BankAccount, conn: &mut PoolConnection<MySql>
) -> Result<(), Box<dyn Error>> {

    let report = BankRecordConverterFactory::get_instance((&bank_account.bank, file))
        .ok_or("no supporting handler found")?
        .convert(file, &bank_account)?;

    validate_bank_record_report(&report)?;

    let mut txn = conn.begin().await?;

    let existing_transactions = bank_transaction_repository::find_transactions_for_time_period(
        report.begin_date, report.end_date, &bank_account, &mut txn).await?;

    let transactions_to_add = find_bank_transactions_to_add(report, existing_transactions);

    for transaction in transactions_to_add {
        bank_transaction_repository::insert(transaction, &mut txn).await?;
    }

    txn.commit().await?;

    Ok(())
}

// TODO: result error handling
pub fn validate_bank_record_report(report: &BankRecordReport) -> Result<(), Box<dyn Error>> {
    if report.begin_date.is_some() && report.end_date.is_some() {
        if report.begin_date.unwrap().cmp(&report.end_date.unwrap()).is_gt() {
            panic!("Begin date cannot be after end date! ({} > {})", report.begin_date.unwrap(), report.end_date.unwrap());
        }
    }

    Ok(())
}

pub fn find_bank_transactions_to_add(
    report: BankRecordReport, existing_transactions: Vec<BankTransaction>
) -> Vec<BankTransaction> {

    let mut transactions_to_add = Vec::new();

    let new_transaction_map = bank_record_util::build_bank_record_transaction_map(report.transactions);
    let existing_transaction_map = bank_record_util::build_bank_record_transaction_map(existing_transactions);

    for (key, mut transactions) in new_transaction_map {
        let existing_transaction_count = existing_transaction_map.get(&key)
            .map_or(0, |transactions| transactions.len());

        for _ in 0..(transactions.len() - existing_transaction_count) {
            let transaction = transactions.remove(0);
            transactions_to_add.push(transaction);
        }
    }

    transactions_to_add
}

pub async fn get_full_transaction_details_by_user(
    user: &User, conn: &mut PoolConnection<MySql>
) -> Result<Vec<FullTransactionDetails>, ApplicationError> {

    let transaction_details_list = bank_transaction_repository::get_transaction_details_by_user(user, conn).await?;

    let mut full_transaction_details_list = Vec::with_capacity(transaction_details_list.len());
    for transaction in transaction_details_list {
        let budget_mappings =
            budget_transaction_mapping_repository::get_details_by_transaction_id(
                transaction.id, conn).await?;
        full_transaction_details_list.push(FullTransactionDetails {
            transaction,
            budget_mappings,
        })
    }

    Ok(full_transaction_details_list)
}

pub async fn get_transaction_details_by_id(
    transaction_id: i64, user: &User, conn: &mut PoolConnection<MySql>
) -> Result<TransactionDetails, Box<dyn Error>> {

    bank_transaction_repository::get_transaction_details_by_id(transaction_id, user, conn).await
        .map_err(|e| e.into_database_error().unwrap().into())
}

pub async fn set_transaction_user_description(
    transaction_id: i64, user_description: Option<String>, user: &User,
    conn: &mut PoolConnection<MySql>
) -> Result<(), Box<dyn Error>> {

    let transaction = bank_transaction_repository::find_editable_transaction_by_id(transaction_id, &user, conn).await?;

    let mut txn = conn.begin().await?;
    let result = bank_transaction_repository::set_user_description(transaction, user_description, &mut txn).await?;
    txn.commit().await?;

    Ok(result)
}

pub async fn set_transaction_comment(
    transaction_id: i64, comment: Option<String>, user: &User, conn: &mut PoolConnection<MySql>
) -> Result<(), Box<dyn Error>> {

    let transaction = bank_transaction_repository::find_editable_transaction_by_id(transaction_id, &user, conn).await?;

    let mut txn = conn.begin().await?;
    let result = bank_transaction_repository::set_comment(transaction, comment, &mut txn).await?;
    txn.commit().await?;

    Ok(result)
}

pub async fn get_top_transaction_groups_by_budget_id(
    budget_id: i64, limit: i32, begin_date: Option<NaiveDate>, end_date: Option<NaiveDate>,
    user: &User, conn: &mut PoolConnection<MySql>
) -> Result<Vec<TransactionGroup>, ApplicationError> {

    let budget = budget_repository::find_by_id(budget_id, &user, conn).await?;

    Ok(bank_transaction_repository::get_top_transaction_groups_for_budget(
        budget, limit, begin_date, end_date, user, conn).await?)
}

pub async fn get_amount_spent_unbudgeted_for_user_and_month(
    user: &User, year: i32, month: u32, conn: &mut PoolConnection<MySql>
) -> Result<i32, Box<dyn Error>> {

    let (begin_date, end_date) = date_util::get_date_range(year, month)?;

    bank_transaction_repository::get_amount_spent_unbudgeted_for_user_and_time_period(
        user, begin_date, end_date, conn).await
        .map_err(|e| e.into_database_error().unwrap().into())
}

pub async fn get_budget_transaction_mapping_details_by_id(
    budget_transaction_mapping_id: i64, user: &User, conn: &mut PoolConnection<MySql>
) -> Result<BudgetTransactionMappingDetails, ApplicationError> {

    let mapping_details =
        budget_transaction_mapping_repository::get_details_by_id(
            budget_transaction_mapping_id, user, conn).await?;

    Ok(mapping_details)
}

pub async fn add_budget_transaction_mapping(
    transaction_id: i64, budget_id: i64, user: &User, amount: i32, behavior: TransactionBehavior,
    conn: &mut PoolConnection<MySql>
) -> Result<i64, ApplicationError> {

    let transaction = bank_transaction_repository::find_editable_transaction_by_id(
        transaction_id, user, conn).await?;
    let budget = budget_repository::find_by_id(budget_id, user, conn).await?;

    let mut txn = conn.begin().await?;

    let mapping = budget_transaction_mapping_repository::insert(
        transaction, budget, amount, behavior, &mut txn).await?;

    txn.commit().await?;

    Ok(mapping)
}

pub async fn update_budget_transaction_mapping(
    budget_transaction_mapping_id: i64, user: &User, amount: i32, behavior: TransactionBehavior,
    conn: &mut PoolConnection<MySql>
) -> Result<(), ApplicationError> {

    let budget_transaction_mapping = budget_transaction_mapping_repository::find_editable_mapping_by_id(budget_transaction_mapping_id, user, conn).await?;

    let mut txn = conn.begin().await?;

    budget_transaction_mapping_repository::update(budget_transaction_mapping, amount, behavior, &mut txn).await?;

    txn.commit().await?;

    Ok(())
}

pub async fn remove_budget_transaction_mapping(
    budget_transaction_mapping_id: i64, user: &User, conn: &mut PoolConnection<MySql>
) -> Result<(), ApplicationError> {

    let budget_transaction_mapping = budget_transaction_mapping_repository::find_editable_mapping_by_id(budget_transaction_mapping_id, user, conn).await?;

    let mut txn = conn.begin().await?;
    budget_transaction_mapping_repository::remove(budget_transaction_mapping, user, &mut txn).await?;
    txn.commit().await?;
    Ok(())
}