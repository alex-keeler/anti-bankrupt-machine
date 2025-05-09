use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use rocket::http::ContentType;
use rocket_db_pools::Connection;
use sqlx::{Acquire};
use crate::models::data::application_error::ApplicationError;

use crate::models::data::bank_record_report::BankRecordReport;
use crate::models::bank_accounts::BankAccount;
use crate::models::bank_transactions::BankTransaction;
use crate::models::budget_transaction_mapping::BudgetTransactionMapping;
use crate::models::data::application_error::ApplicationError::ServiceError;
use crate::models::data::budget_classification_spending_report::BudgetClassificationSpendingReport;
use crate::models::data::budget_spending_report::BudgetSpendingReport;
use crate::models::query::full_transaction_details::FullTransactionDetails;
use crate::models::DB;
use crate::models::query::budget_transaction_mapping_details::BudgetTransactionMappingDetails;
use crate::models::data::spending_report::SpendingReport;
use crate::models::query::transaction_details::TransactionDetails;
use crate::models::users::User;
use crate::repositories::{bank_transaction_repository, budget_repository, budget_transaction_mapping_repository};
use crate::services::converters::BankRecordConverterFactory;
use crate::services::factories::Factory;
use crate::util::{bank_record_util, date_util};

pub async fn build_month_spending_report(year: i32, month: u32, user: &User, conn: &mut Connection<DB>) -> Result<SpendingReport, ApplicationError> {

    let (begin_date, end_date) = date_util::get_date_range(year, month)?;

    let spending_by_budget = budget_transaction_mapping_repository::get_spending_by_budget_for_time_period(Some(&begin_date), Some(&end_date), user, conn).await?;

    let mut classification_map = HashMap::new();
    let mut total_spent_unbudgeted = 0;

    for budget_spend in spending_by_budget {
        if let (Some(title), Some(classification)) = (budget_spend.budget_title, budget_spend.budget_classification) {

            let budget_spending_report = BudgetSpendingReport {
                title,
                total_spent: budget_spend.total_amount,
            };

            classification_map.entry(classification).or_insert(vec![]).push(budget_spending_report);
        } else {
            total_spent_unbudgeted += budget_spend.total_amount;
        }
    }

    let mut classification_spending_reports = vec![];

    for (classification, mut budget_spending_reports) in classification_map.drain() {

        budget_spending_reports.sort_by(|a, b| b.total_spent.cmp(&a.total_spent));

        let total_spent = budget_spending_reports.iter()
            .map(|r| r.total_spent)
            .sum();

        let classification_spending_report = BudgetClassificationSpendingReport {
            classification,
            total_spent,
            budgets: budget_spending_reports,
        };

        classification_spending_reports.push(classification_spending_report);
    };

    classification_spending_reports.sort_by(|a, b| b.total_spent.cmp(&a.total_spent));

    let total_spent = classification_spending_reports.iter()
        .map(|r| r.total_spent)
        .sum::<i32>() + total_spent_unbudgeted;

    let spending_report = SpendingReport {
        total_spent,
        total_spent_unbudgeted,
        budget_classifications: classification_spending_reports,
    };

    Ok(spending_report)
}

pub async fn import_bank_record_file(path: &Path, filename: &str, content_type: &ContentType, _user: User, bank_account: BankAccount, conn: &mut Connection<DB>) -> Result<(), ApplicationError> {

    let report = BankRecordConverterFactory::get_instance((&bank_account.bank, filename, content_type))
        .ok_or("no supporting handler found")?
        .convert(path, &bank_account)?;

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

pub fn validate_bank_record_report(report: &BankRecordReport) -> Result<(), ApplicationError> {
    if report.begin_date.is_some() && report.end_date.is_some() {
        if report.begin_date.unwrap().cmp(&report.end_date.unwrap()).is_gt() {
            return Err(format!("Begin date cannot be after end date! ({} > {})", report.begin_date.unwrap(), report.end_date.unwrap()).into());
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

pub async fn get_full_transaction_details_by_user_and_month(
    user: &User, year: i32, month: u32, conn: &mut Connection<DB>
) -> Result<Vec<FullTransactionDetails>, ApplicationError> {

    let (begin_date, end_date) = date_util::get_date_range(year, month)?;

    let transaction_details_list = bank_transaction_repository::get_transaction_details_by_user_and_time_period(user, begin_date, end_date, conn).await?;

    let mut full_transaction_details_list = Vec::with_capacity(transaction_details_list.len());
    for transaction in transaction_details_list {
        let budget_mappings =
            budget_transaction_mapping_repository::get_details_by_transaction_id(
                transaction.id, user, conn).await?;
        full_transaction_details_list.push(FullTransactionDetails {
            transaction,
            budget_mappings,
        })
    }

    Ok(full_transaction_details_list)
}

pub async fn get_transaction_details_by_id(
    transaction_id: i64, user: &User, conn: &mut Connection<DB>
) -> Result<TransactionDetails, Box<dyn Error>> {

    bank_transaction_repository::get_transaction_details_by_id(transaction_id, user, conn).await
        .map_err(|e| e.into_database_error().unwrap().into())
}


pub async fn set_transaction_user_description(
    transaction_id: i64, user_description: Option<String>, user: &User,
    conn: &mut Connection<DB>
) -> Result<(), Box<dyn Error>> {

    let transaction = bank_transaction_repository::find_editable_transaction_by_id(transaction_id, &user, conn).await?;

    let mut txn = conn.begin().await?;
    let result = bank_transaction_repository::set_user_description(transaction, user_description, &mut txn).await?;
    txn.commit().await?;

    Ok(result)
}

pub async fn set_transaction_comment(
    transaction_id: i64, comment: Option<String>, user: &User, conn: &mut Connection<DB>
) -> Result<(), Box<dyn Error>> {

    let transaction = bank_transaction_repository::find_editable_transaction_by_id(transaction_id, &user, conn).await?;

    let mut txn = conn.begin().await?;
    let result = bank_transaction_repository::set_comment(transaction, comment, &mut txn).await?;
    txn.commit().await?;

    Ok(result)
}

pub async fn get_top_transactions_by_budget_id_and_month(
    budget_id: i64, year: i32, month: u32, limit: Option<i32>, user: &User, conn: &mut Connection<DB>
) -> Result<Vec<TransactionDetails>, ApplicationError> {

    let budget = budget_repository::find_by_id(budget_id, &user, conn).await?;
    let (begin_date, end_date) = date_util::get_date_range(year, month)?;

    Ok(bank_transaction_repository::get_top_transactions_for_budget_and_month(
        budget, begin_date, end_date, limit, user, conn).await?)
}

pub async fn get_amount_spent_unbudgeted_for_user_and_month(
    user: &User, year: i32, month: u32, conn: &mut Connection<DB>
) -> Result<i32, Box<dyn Error>> {

    let (begin_date, end_date) = date_util::get_date_range(year, month)?;

    bank_transaction_repository::get_amount_spent_unbudgeted_for_user_and_time_period(
        user, begin_date, end_date, conn).await
        .map_err(|e| e.into_database_error().unwrap().into())
}

pub async fn get_budget_transaction_mapping_details_by_transaction_id(
    transaction_id: i64, user: &User, conn: &mut Connection<DB>
) -> Result<Vec<BudgetTransactionMappingDetails>, ApplicationError> {

    let mapping_details_list =
        budget_transaction_mapping_repository::get_details_by_transaction_id(
            transaction_id, user, conn).await?;

    Ok(mapping_details_list)
}

pub async fn assign_budget_transaction_mappings(
    transaction_id: i64, mappings: Vec<BudgetTransactionMapping>, user: &User, conn: &mut Connection<DB>
) -> Result<(), ApplicationError> {

    print!("mappings: {:?}", mappings);

    let transaction = bank_transaction_repository::find_editable_transaction_by_id(transaction_id, &user, conn).await?;

    if !verify_all_mappings_belong_to_transaction(&transaction, &mappings) {
        return Err(ServiceError(String::from("Received a mapping that does not belong to transaction")));
    }

    if !verify_budget_transaction_mapping_amount(&transaction, &mappings) {
        return Err(ServiceError(String::from("Total budget transaction mapping amount does not equal transaction amount")));
    }

    let existing_mappings = budget_transaction_mapping_repository::find_all_by_transaction(&transaction, conn).await?;
    let mut remaining_existing_mapping_id_map = existing_mappings.into_iter().map(|m| (m.id.unwrap(), m)).collect::<HashMap<_, _>>();

    let mut txn = conn.begin().await?;

    for new_mapping in mappings {

        let budget = budget_repository::find_by_id_txn(new_mapping.budget_id, &user, &mut txn).await?;

        if new_mapping.id.is_some() && remaining_existing_mapping_id_map.contains_key(&new_mapping.id.unwrap()) {
            let existing_mapping = remaining_existing_mapping_id_map.remove(&new_mapping.id.unwrap()).unwrap();
            budget_transaction_mapping_repository::update(existing_mapping, &transaction, &budget, new_mapping.amount, new_mapping.behavior, &mut txn).await?;
        } else {
            budget_transaction_mapping_repository::insert(&transaction, &budget, new_mapping.amount, new_mapping.behavior, &mut txn).await?;
        }
    }

    for remaining_mapping in remaining_existing_mapping_id_map.into_iter().map(|(_, m)| m) {
        budget_transaction_mapping_repository::remove(remaining_mapping, user, &mut txn).await?;
    }

    txn.commit().await?;

    Ok(())
}

pub fn verify_budget_transaction_mapping_amount(
    transaction: &BankTransaction, mappings: &Vec<BudgetTransactionMapping>
) -> bool {

    let total_mapping_amount: i32 = mappings.iter()
        .map(|mapping| mapping.amount)
        .sum();

    total_mapping_amount == transaction.amount
}

pub fn verify_all_mappings_belong_to_transaction(
    transaction: &BankTransaction, mappings: &Vec<BudgetTransactionMapping>
) -> bool {

    let transaction_id = transaction.id.unwrap();

    for mapping in mappings {
        if transaction_id != mapping.bank_transaction_id {
           return false;
        }
    }

    true
}