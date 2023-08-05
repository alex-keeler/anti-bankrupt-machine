use std::error::Error;
use sqlx::MySql;
use sqlx::pool::PoolConnection;
use crate::models::transaction_categories::TransactionCategoryGroup;

pub async fn find_all(conn: &mut PoolConnection<MySql>) -> Result<Vec<TransactionCategoryGroup>, Box<dyn Error>> {

    let sql = "\
        SELECT * FROM transaction_category_group \
    ";
    let query = sqlx::query_as::<_, TransactionCategoryGroup>(sql);

    let category_group_list = query.fetch_all(conn).await?;
    Ok(category_group_list)
}