use std::collections::HashMap;
use std::error::Error;

use chrono::NaiveDate;
use rocket::fs::TempFile;
use sqlx::{Acquire, MySql};
use sqlx::pool::PoolConnection;

use crate::data::bank_record_report::BankRecordReport;
use crate::models::bank_accounts::BankAccount;
use crate::models::bank_transactions::BankTransaction;
use crate::models::query::transaction_category_details::TransactionCategoryDetails;
use crate::models::query::transaction_category_select::{TransactionCategoryGroupSelect, TransactionCategorySelect};
use crate::models::query::transaction_details::TransactionDetails;
use crate::models::query::transaction_group::TransactionGroup;
use crate::models::transaction_categories::CategoryClassification;
use crate::models::users::User;
use crate::repositories::{bank_transaction_repository, budget_repository, transaction_category_group_repository, transaction_category_repository};
use crate::services::converters::BankRecordConverterFactory;
use crate::services::factories::Factory;
use crate::util::bank_record_util;

pub async fn import_bank_record_file(file: &TempFile<'_>, _user: User, bank_account: BankAccount, conn: &mut PoolConnection<MySql>) -> Result<(), Box<dyn Error>> {
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

pub fn find_bank_transactions_to_add(report: BankRecordReport, existing_transactions: Vec<BankTransaction>) -> Vec<BankTransaction> {
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

pub async fn get_transaction_details_by_user(user: &User, conn: &mut PoolConnection<MySql>) -> Result<Vec<TransactionDetails>, Box<dyn Error>> {
    bank_transaction_repository::get_transaction_details_by_user(user, conn).await
}

pub async fn get_transaction_details_by_id(transaction_id: i64, user: &User, conn: &mut PoolConnection<MySql>) -> Result<TransactionDetails, Box<dyn Error>> {
    bank_transaction_repository::get_transaction_details_by_id(transaction_id, user, conn).await
}

pub async fn get_transaction_category_group_select_list(user: &User, conn: &mut PoolConnection<MySql>) -> Result<Vec<TransactionCategoryGroupSelect>, Box<dyn Error>> {
    let category_list = transaction_category_repository::find_all(user, conn).await?;
    let category_group_list = transaction_category_group_repository::find_all(conn).await?;

    let mut category_select_map = HashMap::new();
    for category in category_list {
        let category_select = TransactionCategorySelect {
            id: category.id.unwrap(),
            title: category.title,
        };
        category_select_map
            .entry(category.category_group_id)
            .or_insert(Vec::new())
            .push(category_select);
    }

    let mut category_group_select_list = Vec::with_capacity(category_group_list.len());
    for category_group in category_group_list {
        let category_group_select = TransactionCategoryGroupSelect {
            id: category_group.id.unwrap(),
            title: category_group.title,
            classification: category_group.classification,
            categories: category_select_map.remove(&category_group.id.unwrap()).unwrap_or(Vec::new()),
        };

        if !category_group_select.categories.is_empty() {
            category_group_select_list.push(category_group_select);
        }
    }

    Ok(category_group_select_list)
}

pub async fn set_transaction_category(transaction_id: i64, category_id: Option<i64>, user: &User, conn: &mut PoolConnection<MySql>) -> Result<(), Box<dyn Error>> {
    let transaction = bank_transaction_repository::find_editable_transaction_by_id(transaction_id, &user, conn).await?;
    let category = if category_id.is_some() { Some(transaction_category_repository::find_by_id(category_id.unwrap(), user, conn).await?) } else { None };

    let mut txn = conn.begin().await?;
    let result = bank_transaction_repository::set_category(transaction, category, &mut txn).await?;
    txn.commit().await?;

    Ok(result)
}

pub async fn set_transaction_user_description(transaction_id: i64, user_description: Option<String>, user: &User, conn: &mut PoolConnection<MySql>) -> Result<(), Box<dyn Error>> {
    let transaction = bank_transaction_repository::find_editable_transaction_by_id(transaction_id, &user, conn).await?;

    let mut txn = conn.begin().await?;
    let result = bank_transaction_repository::set_user_description(transaction, user_description, &mut txn).await?;
    txn.commit().await?;

    Ok(result)
}

pub async fn set_transaction_comment(transaction_id: i64, comment: Option<String>, user: &User, conn: &mut PoolConnection<MySql>) -> Result<(), Box<dyn Error>> {
    let transaction = bank_transaction_repository::find_editable_transaction_by_id(transaction_id, &user, conn).await?;

    let mut txn = conn.begin().await?;
    let result = bank_transaction_repository::set_comment(transaction, comment, &mut txn).await?;
    txn.commit().await?;

    Ok(result)
}

pub async fn set_transaction_hide_from_calculations(transaction_id: i64, hide_from_calculations: bool, user: &User, conn: &mut PoolConnection<MySql>) -> Result<(), Box<dyn Error>> {
    let transaction = bank_transaction_repository::find_editable_transaction_by_id(transaction_id, &user, conn).await?;

    let mut txn = conn.begin().await?;
    let result = bank_transaction_repository::set_hide_from_calculations(transaction, hide_from_calculations, &mut txn).await?;
    txn.commit().await?;

    Ok(result)
}

pub async fn set_transaction_contribution_budget(transaction_id: i64, budget_id: Option<i64>, user: &User, conn: &mut PoolConnection<MySql>) -> Result<(), Box<dyn Error>> {
    let transaction = bank_transaction_repository::find_editable_transaction_by_id(transaction_id, &user, conn).await?;

    let mut txn = conn.begin().await?;
    let result = bank_transaction_repository::set_contribution_budget(transaction, budget_id, &mut txn).await?;
    txn.commit().await?;

    Ok(result)
}

pub async fn get_top_transaction_groups_by_budget_id(budget_id: i64, limit: i32, begin_date: Option<NaiveDate>, end_date: Option<NaiveDate>, user: &User, conn: &mut PoolConnection<MySql>) -> Result<Vec<TransactionGroup>, Box<dyn Error>> {
    let budget = budget_repository::find_by_id(budget_id, conn).await?;
    bank_transaction_repository::get_top_transaction_groups_for_budget(budget, limit, begin_date, end_date, user, conn).await
}

pub async fn get_transaction_category_details_list_by_classification_and_month(classification: CategoryClassification, year: i32, month: u32, user: &User, conn: &mut PoolConnection<MySql>) -> Result<Vec<TransactionCategoryDetails>, Box<dyn Error>> {
    transaction_category_repository::get_details_list_for_classification_and_month(classification, year, month, user, conn).await
}