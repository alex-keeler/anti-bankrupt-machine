use std::collections::HashMap;
use std::error::Error;
use sqlx::MySql;
use sqlx::pool::PoolConnection;
use crate::models::transaction_categories::TransactionCategory;

pub async fn find_all(conn: &mut PoolConnection<MySql>) -> Result<Vec<TransactionCategory>, Box<dyn Error>> {

    let sql = "\
        SELECT * FROM transaction_category \
    ";
    let query = sqlx::query_as::<_, TransactionCategory>(sql);

    let category_list = query.fetch_all(conn).await?;
    Ok(category_list)
}

pub async fn find_by_id(category_id: i64, conn: &mut PoolConnection<MySql>) -> Result<TransactionCategory, Box<dyn Error>> {

    let sql = "\
        SELECT * FROM transaction_category \
        WHERE id = ?
    ";
    let query = sqlx::query_as::<_, TransactionCategory>(sql)
        .bind(category_id);

    let category = query.fetch_one(conn).await?;
    Ok(category)
}