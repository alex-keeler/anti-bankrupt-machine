use std::collections::HashMap;
use std::error::Error;
use sqlx::MySql;
use sqlx::pool::PoolConnection;
use crate::models::transaction_categories::TransactionCategory;
use crate::models::users::User;

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