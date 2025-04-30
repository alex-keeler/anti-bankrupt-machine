
use chrono::NaiveDate;
use sqlx::{Error, MySql, Row, Transaction};
use rocket_db_pools::Connection;

use crate::models::bank_account_users::BankAccountUserRole;
use crate::models::bank_accounts::BankAccount;
use crate::models::bank_transactions::BankTransaction;
use crate::models::budget::Budget;
use crate::models::DB;
use crate::models::query::transaction_details::TransactionDetails;
use crate::models::users::User;
use crate::repositories::repository_util::{get_executor_conn, get_executor_txn};

pub async fn find_transactions_for_time_period(begin_date: Option<NaiveDate>, end_date: Option<NaiveDate>, bank_account: &BankAccount, txn: &mut Transaction<'_, MySql>) -> Result<Vec<BankTransaction>, Error> {

    let mut sql = String::from("\
        SELECT bt.* FROM bank_transaction bt \
        WHERE bt.bank_account_id = ? \
    ");

    if begin_date.is_some() {
        sql = sql + " AND bt.date >= ? ";
    }

    if end_date.is_some() {
        sql = sql + " AND bt.date <= ? ";
    }

    let mut query = sqlx::query_as::<_, BankTransaction>(sql.as_str())
        .bind(&bank_account.id);

    if begin_date.is_some() {
        query = query.bind(begin_date.unwrap());
    }

    if end_date.is_some() {
        query = query.bind(end_date.unwrap());
    }

    let transactions = query.fetch_all(get_executor_txn(txn)).await?;
    Ok(transactions)
}

pub async fn insert(transaction: BankTransaction, txn: &mut Transaction<'_, MySql>) -> Result<(), Error> {

    let sql = "\
        INSERT INTO bank_transaction \
        (date, bank_description, user_description, comment, amount, bank_account_id)
        VALUES (?, ?, ?, ?, ?, ?)
    ";

    let query = sqlx::query(sql)
        .bind(transaction.date)
        .bind(transaction.bank_description)
        .bind(transaction.user_description)
        .bind(transaction.comment)
        .bind(transaction.amount)
        .bind(transaction.bank_account_id);

    let _ = query.execute(get_executor_txn(txn)).await?;

    Ok(())
}

pub async fn get_transaction_details_by_user_and_time_period(user: &User, begin_date: NaiveDate, end_date: NaiveDate, conn: &mut Connection<DB>) -> Result<Vec<TransactionDetails>, Error> {

    let sql = "\
            SELECT bt.id, bt.date, bt.bank_description, bt.user_description, bt.comment, \
                   bt.amount, ba.title AS bank_account_title, \
                   CAST(COUNT(btm.id) AS UNSIGNED) AS count_budget_mappings, \
                   CASE WHEN COUNT(btm.id) = 1 THEN \
                        (SELECT title FROM budget b WHERE b.id = MIN(btm.budget_id)) \
                   ELSE NULL END AS single_budget_title \
            FROM users u \
                JOIN bank_account_user bau ON bau.user_id = u.id \
                JOIN bank_account ba ON ba.id = bau.bank_account_id \
                JOIN bank_transaction bt ON bt.bank_account_id = ba.id \
                LEFT JOIN budget_transaction_mapping btm ON btm.bank_transaction_id = bt.id \
            WHERE u.id = ? \
                AND bt.date >= ? \
                AND bt.date < ? \
            GROUP BY bt.id \
            ORDER BY bt.date DESC, bt.id ASC \
    ";

    let query = sqlx::query_as::<_, TransactionDetails>(sql)
        .bind(user.id)
        .bind(begin_date)
        .bind(end_date);

    let transaction_details = query.fetch_all(get_executor_conn(conn)).await?;
    Ok(transaction_details)
}

pub async fn get_transaction_details_by_id(transaction_id: i64, user: &User, conn: &mut Connection<DB>) -> Result<TransactionDetails, Error> {

    let sql = "\
            SELECT bt.id, bt.date, bt.bank_description, bt.user_description, bt.comment, bt.amount, \
                ba.title AS bank_account_title \
            FROM bank_transaction bt \
                JOIN bank_account ba ON ba.id = bt.bank_account_id \
                JOIN bank_account_user bau ON bau.bank_account_id = ba.id \
                JOIN users u ON u.id = bau.user_id \
            WHERE bt.id = ? \
                AND u.id = ? \
    ";

    let query = sqlx::query_as::<_, TransactionDetails>(sql)
        .bind(transaction_id)
        .bind(user.id);

    let transaction_details = query.fetch_one(get_executor_conn(conn)).await?;
    Ok(transaction_details)
}

pub async fn set_user_description(transaction: BankTransaction, user_description: Option<String>, txn: &mut Transaction<'_, MySql>) -> Result<(), Error> {

    let sql = "\
            UPDATE bank_transaction bt
            SET bt.user_description = ?
            WHERE bt.id = ?
    ";

    // TODO: "NotPersisted" error
    let query = sqlx::query(sql)
        .bind(user_description)
        .bind(transaction.id.unwrap());

    let _ = query.execute(get_executor_txn(txn)).await?;

    Ok(())
}

pub async fn set_comment(transaction: BankTransaction, comment: Option<String>, txn: &mut Transaction<'_, MySql>) -> Result<(), Error> {

    let sql = "\
            UPDATE bank_transaction bt
            SET bt.comment = ?
            WHERE bt.id = ?
    ";

    // TODO: "NotPersisted" error
    let query = sqlx::query(sql)
        .bind(comment)
        .bind(transaction.id.unwrap());

    let _ = query.execute(get_executor_txn(txn)).await?;

    Ok(())
}

pub async fn find_editable_transaction_by_id(transaction_id: i64, user: &User, conn: &mut Connection<DB>) -> Result<BankTransaction, Error> {

    let sql = "\
            SELECT bt.* FROM bank_transaction bt
                JOIN bank_account ba ON ba.id = bt.bank_account_id
                JOIN bank_account_user bau ON bau.bank_account_id = ba.id
                JOIN users u ON u.id = bau.user_id
            WHERE bt.id = ?
                AND u.id = ?
                AND bau.role IN (?, ?)
    ";

    let query = sqlx::query_as::<_, BankTransaction>(sql)
        .bind(transaction_id)
        .bind(user.id)
        .bind(BankAccountUserRole::Owner)
        .bind(BankAccountUserRole::Editor);

    let transaction = query.fetch_one(get_executor_conn(conn)).await?;

    Ok(transaction)
}

pub async fn get_top_transactions_for_budget_and_month(
    budget: Budget, begin_date: NaiveDate, end_date: NaiveDate, limit: Option<i32>,
    user: &User, conn: &mut Connection<DB>
) -> Result<Vec<TransactionDetails>, Error> {

    let mut sql = String::from("\
            SELECT bt.id, bt.date, bt.bank_description, bt.user_description, bt.comment, bt.amount, ba.title as bank_account_title \
            FROM bank_transaction bt \
                JOIN budget_transaction_mapping btm ON btm.bank_transaction_id = bt.id \
                JOIN budget b ON b.id = btm.budget_id \
                JOIN bank_account ba ON ba.id = bt.bank_account_id \
                JOIN bank_account_user bau ON bau.bank_account_id = ba.id \
                JOIN users u ON u.id = bau.user_id \
            WHERE b.id = ? \
                AND u.id = ? \
                AND bt.date >= ? \
                AND bt.date < ? \
    ");
    // sorted by ascending amount since results are negative
    sql = sql + " ORDER BY bt.amount ";

    if let Some(_) = &limit {
        sql = sql + " LIMIT ? ";
    }

    let mut query = sqlx::query_as::<_, TransactionDetails>(sql.as_str())
        .bind(budget.id)
        .bind(user.id)
        .bind(begin_date)
        .bind(end_date);

    if let Some(limit) = &limit {
        query = query.bind(limit);
    }

    let transactions = query.fetch_all(get_executor_conn(conn)).await?;

    Ok(transactions)
}

pub async fn get_amount_spent_unbudgeted_for_user_and_time_period(user: &User, begin_date: NaiveDate, end_date: NaiveDate, conn: &mut Connection<DB>) -> Result<i32, Error> {

    let sql = "\
            SELECT CAST(IFNULL(-1 * SUM(bt.amount), 0) AS INT) AS amount_spent \
            FROM bank_transaction bt \
                JOIN bank_account ba ON ba.id = bt.bank_account_id \
                JOIN bank_account_user bau ON bau.bank_account_id = ba.id \
                JOIN users u ON u.id = bau.user_id \
            WHERE bt.date >= ? \
                AND bt.date < ? \
                AND u.id = ? \
                AND NOT EXISTS ( \
                    SELECT btm.* FROM budget_transaction_mapping btm \
                        JOIN budget b ON b.id = btm.budget_id \
                    WHERE btm.bank_transaction_id = bt.id \
                        AND b.user_id = u.id \
                ) \
        ";

    let query = sqlx::query(sql)
        .bind(begin_date)
        .bind(end_date)
        .bind(user.id);

    let row = query.fetch_one(get_executor_conn(conn)).await?;
    Ok(row.get("amount_spent"))
}