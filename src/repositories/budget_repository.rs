use chrono::NaiveDate;
use rocket_db_pools::Pool;

use sqlx::{MySql, Error, query_as, Row};
use sqlx::pool::PoolConnection;

use crate::models::budget::Budget;
use crate::models::query::budget_details::BudgetDetails;
use crate::models::users::User;

pub async fn find_by_id(id: i64, user: &User, conn: &mut PoolConnection<MySql>) -> Result<Budget, Error> {
    let sql = "\
            SELECT b.* FROM budget b \
            WHERE b.id = ? \
                AND b.user_id = ? \
    ";

    let query = sqlx::query_as::<_, Budget>(sql)
        .bind(id)
        .bind(user.id);

    let budget = query.fetch_one(conn).await?;
    Ok(budget)
}

pub async fn find_all_by_user(user: &User, conn: &mut PoolConnection<MySql>) -> Result<Vec<Budget>, Error> {
    let sql = "\
            SELECT b.* FROM budget b \
                JOIN users u ON u.id = b.user_id \
            WHERE u.id = ? \
            ORDER BY b.classification, b.sort_order \
    ";

    let query = query_as::<_, Budget>(sql)
        .bind(user.id);

    let budget_list = query.fetch_all(conn).await?;
    Ok(budget_list)
}

pub async fn find_all_visible_by_user(user: &User, conn: &mut PoolConnection<MySql>) -> Result<Vec<Budget>, Error> {
    let sql = "\
            SELECT b.* FROM budget b \
                JOIN users u ON u.id = b.user_id \
            WHERE u.id = ? \
                AND NOT b.hidden \
            ORDER BY b.classification, b.sort_order \
    ";

    let query = query_as::<_, Budget>(sql)
        .bind(user.id);

    let budget_list = query.fetch_all(conn).await?;
    Ok(budget_list)
}

pub async fn get_details_list_for_user_and_time_period(user: &User, begin_date: NaiveDate, end_date: NaiveDate, conn: &mut PoolConnection<MySql>) -> Result<Vec<BudgetDetails>, Error> {
    let sql = "\
            SELECT b.id, b.title, b.classification, b.spending_type, b.cumulative, b.rollover, \
                bh.amount, bh.time_months, bh.payment_date, bh.recur_count \
            FROM budget b \
                JOIN users u ON u.id = b.user_id \
                JOIN budget_history bh ON bh.budget_id = b.id \
            WHERE u.id = ? \
                AND DATE_SUB(bh.payment_date, INTERVAL bh.time_months MONTH) < ? \
                AND (bh.recur_count IS NULL OR \
                    DATE_ADD(bh.payment_date, INTERVAL (bh.recur_count * bh.time_months) MONTH) >= ?) \
    ";

    let query = sqlx::query_as::<_, BudgetDetails>(sql)
        .bind(user.id)
        .bind(begin_date)
        .bind(end_date);

    let budget_details_list = query.fetch_all(conn).await?;
    Ok(budget_details_list)
}