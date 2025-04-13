use chrono::NaiveDate;
use rocket_db_pools::Connection;

use sqlx::{Error, query_as};

use crate::models::budget::{Budget, BudgetHistory};
use crate::models::DB;
use crate::repositories::repository_util::get_executor_conn;

pub async fn find_applicable_by_budget(budget: &Budget, begin_date: &NaiveDate, end_date: &NaiveDate, conn: &mut Connection<DB>) -> Result<Vec<BudgetHistory>, Error> {
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

    let history_list = query.fetch_all(get_executor_conn(conn)).await?;
    Ok(history_list)
}