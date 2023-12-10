// let transactions = bank_transaction_repository::find_by_budget_and_time_period(
//             &budget, Some(&cycle_begin_date), Some(&end_date), conn).await
//             .map_err(|e| e.into_database_error().unwrap())?;

use chrono::NaiveDate;
use sqlx::{Error, MySql};
use sqlx::pool::PoolConnection;
use crate::models::budget::Budget;
use crate::models::budget_transaction_mapping::BudgetTransactionMapping;
use crate::models::query::budget_transaction_mapping_details::BudgetTransactionMappingDetails;

pub async fn find_by_budget_and_time_period(
    budget: &Budget, begin_date: Option<&NaiveDate>, end_date: Option<&NaiveDate>,
    conn: &mut PoolConnection<MySql>,
) -> Result<Vec<BudgetTransactionMapping>, Error> {

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
        sql = sql + " AND bt.date <= ? ";
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

    let transactions = query.fetch_all(conn).await?;
    Ok(transactions)
}

pub async fn get_budget_transaction_mapping_details_by_transaction_id(
    bank_transaction_id: i64, conn: &mut PoolConnection<MySql>
) -> Result<Vec<BudgetTransactionMappingDetails>, Error> {

    let sql = "\
            SELECT btm.id, btm.bank_transaction_id, btm.budget_id, b.title AS budget_title, \
                btm.amount, btm.behavior \
            FROM budget_transaction_mapping btm \
                JOIN budget b ON b.id = btm.budget_id \
            WHERE btm.bank_transaction_id = ? \
        ";

    let query = sqlx::query_as::<_, BudgetTransactionMappingDetails>(sql)
        .bind(bank_transaction_id);

    let details = query.fetch_all(conn).await?;
    Ok(details)
}