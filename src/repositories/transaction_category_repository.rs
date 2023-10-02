use std::collections::HashMap;
use std::error::Error;
use chrono::NaiveDate;
use sqlx::MySql;
use sqlx::pool::PoolConnection;
use crate::models::query::transaction_category_details::TransactionCategoryDetails;
use crate::models::transaction_categories::{CategoryClassification, TransactionCategory, TransactionCategoryGroup};
use crate::models::users::User;
use crate::util::date_util;

pub async fn find_all(user: &User, conn: &mut PoolConnection<MySql>) -> Result<Vec<TransactionCategory>, Box<dyn Error>> {

    let sql = "\
        SELECT * FROM transaction_category \
        WHERE user_id = ? \
    ";
    let query = sqlx::query_as::<_, TransactionCategory>(sql)
        .bind(user.id);

    let category_list = query.fetch_all(conn).await?;
    Ok(category_list)
}

pub async fn find_by_id(category_id: i64, user: &User, conn: &mut PoolConnection<MySql>) -> Result<TransactionCategory, Box<dyn Error>> {

    let sql = "\
        SELECT * FROM transaction_category \
        WHERE id = ? \
            AND user_id = ? \
    ";
    let query = sqlx::query_as::<_, TransactionCategory>(sql)
        .bind(category_id)
        .bind(user.id);

    let category = query.fetch_one(conn).await?;
    Ok(category)
}

pub async fn get_details_list_for_classification_and_month(classification: CategoryClassification, year: i32, month: u32, user: &User, conn: &mut PoolConnection<MySql>) -> Result<Vec<TransactionCategoryDetails>, Box<dyn Error>> {

    let sql = "\
        SELECT tc.id, tc.title, tcg.id AS category_group_id, tcg.title AS category_group_title, \
            CAST(IFNULL(SUM(bt.amount), 0) AS INTEGER) AS total_amount \
        FROM transaction_category tc \
            JOIN users u ON u.id = tc.user_id \
            JOIN transaction_category_group tcg ON tcg.id = tc.category_group_id \
            LEFT JOIN bank_transaction bt ON bt.category_id = tc.id \
                AND bt.date >= ? \
                AND bt.date < ? \
        WHERE tcg.classification = ? \
            AND u.id = ? \
    ";

    let (begin_date, end_date) = date_util::get_date_range(year, month)?;

    let query = sqlx::query_as::<_, TransactionCategoryDetails>(sql)
        .bind(begin_date)
        .bind(end_date)
        .bind(classification)
        .bind(user.id);

    let details_list = query.fetch_all(conn).await?;
    Ok(details_list)
}