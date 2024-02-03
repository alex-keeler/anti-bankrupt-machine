use chrono::NaiveDate;

use sqlx::{MySql, Error, query_as};
use sqlx::pool::PoolConnection;

use crate::models::budget::{Budget, BudgetHistory};

pub async fn find_applicable_by_budget(budget: &Budget, begin_date: &NaiveDate, end_date: &NaiveDate, conn: &mut PoolConnection<MySql>) -> Result<Vec<BudgetHistory>, Error> {
    let sql = "\
            SELECT bh.* \
            FROM budget b \
                JOIN budget_history bh ON bh.budget_id = b.id \
            WHERE b.id = ? \
                AND bh.start_date <= ? \
                AND (b.cumulative OR bh.recur_count IS NULL OR bh.months_until_completion IS NULL \
                    OR DATE_ADD(bh.start_date, INTERVAL ((bh.recur_count + 1) * bh.months_until_completion) MONTH) >= ?) \
    ";

    let query = query_as::<_, BudgetHistory>(sql)
        .bind(budget.id)
        .bind(begin_date)
        .bind(end_date);

    let history_list = query.fetch_all(conn).await?;
    Ok(history_list)
}