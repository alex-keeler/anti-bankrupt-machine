use std::error::Error;
use std::io::ErrorKind::NotFound;
use chrono::NaiveDate;
use num_format::Locale::tr;
use rocket::execute;
use rocket_db_pools::Pool;
use sqlx::{MySql, Transaction};
use sqlx::pool::PoolConnection;
use crate::models::bank_account_users::BankAccountUserRole;
use crate::models::bank_accounts::BankAccount;
use crate::models::bank_transactions::BankTransaction;
use crate::models::query::transaction_details::TransactionDetails;
use crate::models::transaction_categories::TransactionCategory;
use crate::models::users::User;

pub async fn find_transactions_for_time_period(
        begin_date: Option<NaiveDate>, end_date: Option<NaiveDate>,
        bank_account: &BankAccount, txn: &mut Transaction<'_, MySql>
) -> Result<Vec<BankTransaction>, Box<dyn Error>> {

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

    let transactions = query.fetch_all(txn).await?;
    Ok(transactions)
}

pub async fn insert(transaction: BankTransaction, txn: &mut Transaction<'_, MySql>) -> Result<(), Box<dyn Error>> {

    let sql = "\
        INSERT INTO bank_transaction \
        (date, bank_description, user_description, comment, amount, bank_account_id, category_id, saving_goal_id)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
    ";

    let query = sqlx::query(sql)
        .bind(transaction.date)
        .bind(transaction.bank_description)
        .bind(transaction.user_description)
        .bind(transaction.comment)
        .bind(transaction.amount)
        .bind(transaction.bank_account_id)
        .bind(transaction.category_id)
        .bind(transaction.saving_goal_id);

    let _ = query.execute(txn).await?;

    Ok(())
}

pub async fn get_transaction_details_by_user(user: User, conn: &mut PoolConnection<MySql>) -> Result<Vec<TransactionDetails>, Box<dyn Error>> {

    let sql = "\
            SELECT bt.id, bt.date, bt.bank_description, bt.user_description, bt.comment, bt.amount, \
                   tc.id AS category_id, IF(tc.title IS NULL, tcg.title, CONCAT(tcg.title, ' - ', tc.title)) AS category_title, \
                   tcg.classification as category_classification,sg.id AS saving_goal_id, \
                   sg.title AS saving_goal_title, ba.title AS bank_account_title \
            FROM bank_account_user bau \
                JOIN bank_account ba ON ba.id = bau.bank_account_id \
                JOIN users u ON u.id = bau.user_id \
                JOIN bank_transaction bt ON bt.bank_account_id = ba.id \
                LEFT JOIN transaction_category tc ON tc.id = bt.category_id \
                LEFT JOIN transaction_category_group tcg ON tcg.id = tc.category_group_id \
                LEFT JOIN saving_goal sg ON sg.id = bt.saving_goal_id \
            WHERE u.id = ? \
            ORDER BY bt.date DESC, bt.id ASC \
    ";

    let query = sqlx::query_as::<_, TransactionDetails>(sql)
        .bind(user.id);

    let transaction_details = query.fetch_all(conn).await?;
    Ok(transaction_details)
}

pub async fn get_transaction_details_by_id(transaction_id: i64, user: &User, conn: &mut PoolConnection<MySql>) -> Result<TransactionDetails, Box<dyn Error>> {

    let sql = "\
            SELECT bt.id, bt.date, bt.bank_description, bt.user_description, bt.comment, bt.amount, \
                   tc.id AS category_id, IF(tc.title IS NULL, tcg.title, CONCAT(tcg.title, ' - ', tc.title)) AS category_title, \
                   tcg.classification as category_classification,sg.id AS saving_goal_id, \
                   sg.title AS saving_goal_title, ba.title AS bank_account_title \
            FROM bank_transaction bt \
                JOIN bank_account ba ON ba.id = bt.bank_account_id \
                JOIN bank_account_user bau ON bau.bank_account_id = ba.id \
                JOIN users u ON u.id = bau.user_id \
                LEFT JOIN transaction_category tc ON tc.id = bt.category_id \
                LEFT JOIN transaction_category_group tcg ON tcg.id = tc.category_group_id \
                LEFT JOIN saving_goal sg ON sg.id = bt.saving_goal_id \
            WHERE bt.id = ? \
                AND u.id = ? \
    ";

    let query = sqlx::query_as::<_, TransactionDetails>(sql)
        .bind(transaction_id)
        .bind(user.id);

    let transaction_details = query.fetch_one(conn).await?;
    Ok(transaction_details)
}

pub async fn set_category(transaction: BankTransaction, category: Option<TransactionCategory>, txn: &mut Transaction<'_, MySql>) -> Result<(), Box<dyn Error>> {

    let sql = "\
            UPDATE bank_transaction bt
            SET bt.category_id = ?
            WHERE bt.id = ?
    ";

    // TODO: "NotPersisted" error
    let query = sqlx::query(sql)
        .bind(category.map(|category| category.id.unwrap()))
        .bind(transaction.id.unwrap());

    let _ = query.execute(txn).await?;

    Ok(())
}

pub async fn set_user_description(transaction: BankTransaction, user_description: Option<String>, txn: &mut Transaction<'_, MySql>) -> Result<(), Box<dyn Error>> {

    let sql = "\
            UPDATE bank_transaction bt
            SET bt.user_description = ?
            WHERE bt.id = ?
    ";

    // TODO: "NotPersisted" error
    let query = sqlx::query(sql)
        .bind(user_description)
        .bind(transaction.id.unwrap());

    let _ = query.execute(txn).await?;

    Ok(())
}

pub async fn set_comment(transaction: BankTransaction, comment: Option<String>, txn: &mut Transaction<'_, MySql>) -> Result<(), Box<dyn Error>> {

    let sql = "\
            UPDATE bank_transaction bt
            SET bt.comment = ?
            WHERE bt.id = ?
    ";

    // TODO: "NotPersisted" error
    let query = sqlx::query(sql)
        .bind(comment)
        .bind(transaction.id.unwrap());

    let _ = query.execute(txn).await?;

    Ok(())
}

pub async fn find_editable_transaction_by_id(transaction_id: i64, user: &User, conn: &mut PoolConnection<MySql>) -> Result<BankTransaction, Box<dyn Error>> {

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

    let transaction = query.fetch_one(conn).await?;

    Ok(transaction)
}