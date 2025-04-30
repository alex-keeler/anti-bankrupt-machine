use chrono::NaiveDate;
use rocket_db_pools::Connection;
use sqlx::{Error, MySql, Transaction};
use crate::models::bank_account_users::BankAccountUserRole;
use crate::models::bank_transactions::BankTransaction;
use crate::models::budget::Budget;
use crate::models::budget_transaction_mapping::{BudgetTransactionMapping, TransactionBehavior};
use crate::models::DB;
use crate::models::query::budget_transaction_mapping_details::BudgetTransactionMappingDetails;
use crate::models::users::User;
use crate::repositories::repository_util::{get_executor_conn, get_executor_txn};

pub async fn find_by_budget_and_time_period(budget: &Budget, begin_date: Option<&NaiveDate>, end_date: Option<&NaiveDate>, conn: &mut Connection<DB>) -> Result<Vec<BudgetTransactionMapping>, Error> {

    let mut sql = String::from("\
        SELECT btm.* \
        FROM budget_transaction_mapping btm \
            JOIN bank_transaction bt ON bt.id = btm.bank_transaction_id \
            JOIN budget b ON b.id = btm.budget_id \
        WHERE b.id = ? \
    ");

    if begin_date.is_some() {
        sql = sql + " AND bt.date >= ? ";
    }

    if end_date.is_some() {
        sql = sql + " AND bt.date < ? ";
    }

    sql = sql + " ORDER BY bt.amount ASC ";

    let mut query = sqlx::query_as::<_, BudgetTransactionMapping>(sql.as_str())
        .bind(&budget.id);

    if begin_date.is_some() {
        query = query.bind(begin_date.unwrap());
    }

    if end_date.is_some() {
        query = query.bind(end_date.unwrap());
    }

    let transactions = query.fetch_all(get_executor_conn(conn)).await?;
    Ok(transactions)
}

pub async fn get_details_by_transaction_id(bank_transaction_id: i64, user: &User, conn: &mut Connection<DB>) -> Result<Vec<BudgetTransactionMappingDetails>, Error> {

    let sql = "\
            SELECT btm.id, btm.bank_transaction_id, btm.budget_id, b.title AS budget_title, \
                btm.amount, btm.behavior \
            FROM budget_transaction_mapping btm \
                JOIN budget b ON b.id = btm.budget_id \
                JOIN bank_transaction bt ON bt.id = btm.bank_transaction_id \
                JOIN bank_account ba ON ba.id = bt.bank_account_id \
                JOIN bank_account_user bau ON bau.bank_account_id = ba.id \
            WHERE bt.id = ? \
                AND b.user_id = ? \
                AND bau.user_id = ? \
        ";

    let query = sqlx::query_as::<_, BudgetTransactionMappingDetails>(sql)
        .bind(bank_transaction_id)
        .bind(user.id)
        .bind(user.id);

    let details = query.fetch_all(get_executor_conn(conn)).await?;
    Ok(details)
}

pub async fn find_all_by_transaction(transaction: &BankTransaction, conn: &mut Connection<DB>) -> Result<Vec<BudgetTransactionMapping>, Error> {

    let sql = "\
            SELECT btm.* \
            FROM budget_transaction_mapping btm \
            WHERE btm.bank_transaction_id = ? \
        ";

    let query = sqlx::query_as::<_, BudgetTransactionMapping>(sql)
        .bind(transaction.id);

    let mappings = query.fetch_all(get_executor_conn(conn)).await?;
    Ok(mappings)
}

pub async fn insert(bank_transaction: &BankTransaction, budget: &Budget, amount: i32, behavior: TransactionBehavior, txn: &mut Transaction<'_, MySql>) -> Result<i64, Error> {

    let insert_sql = "\
            INSERT INTO budget_transaction_mapping \
            (bank_transaction_id, budget_id, amount, behavior) \
            VALUES (?, ?, ?, ?) \
        ";

    let insert_query = sqlx::query(insert_sql)
        .bind(bank_transaction.id.unwrap())
        .bind(budget.id.unwrap())
        .bind(amount)
        .bind(behavior);

    let id = insert_query
        .execute(get_executor_txn(txn))
        .await?
        .last_insert_id();

    Ok(id as i64)
}

pub async fn update(budget_transaction_mapping: BudgetTransactionMapping, transaction: &BankTransaction, budget: &Budget, amount: i32, behavior: TransactionBehavior, txn: &mut Transaction<'_, MySql>) -> Result<(), Error> {

    let insert_sql = "\
            UPDATE budget_transaction_mapping btm \
            SET btm.bank_transaction_id = ?, btm.budget_id = ?, btm.amount = ?, btm.behavior = ? \
            WHERE btm.id = ? \
        ";

    let update_query = sqlx::query(insert_sql)
        .bind(transaction.id)
        .bind(budget.id)
        .bind(amount)
        .bind(behavior)
        .bind(budget_transaction_mapping.id.unwrap());

    update_query.execute(get_executor_txn(txn)).await?;

    Ok(())
}

pub async fn remove(budget_transaction_mapping: BudgetTransactionMapping, user: &User, txn: &mut Transaction<'_, MySql>) -> Result<(), Error> {

    let sql = "\
            DELETE btm FROM budget_transaction_mapping btm \
                JOIN budget b ON b.id = btm.budget_id \
                JOIN bank_transaction bt ON bt.id = btm.bank_transaction_id \
                JOIN bank_account ba ON ba.id = bt.bank_account_id \
                JOIN bank_account_user bau ON bau.bank_account_id = ba.id \
            WHERE btm.id = ? \
                AND b.user_id = ? \
                AND bau.user_id = ? \
                AND bau.role IN (?, ?) \
        ";

    let query = sqlx::query(sql)
        .bind(budget_transaction_mapping.id)
        .bind(user.id)
        .bind(user.id)
        .bind(BankAccountUserRole::Owner)
        .bind(BankAccountUserRole::Editor);

    query.execute(get_executor_txn(txn)).await?;

    Ok(())
}