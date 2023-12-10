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

/*
pub async fn get_details_list_for_user_and_month(user: &User, year: i32, month: u32, conn: &mut PoolConnection<MySql>) -> Result<Vec<BudgetDetails>, Box<dyn Error>> {
    let sql = "\
            SELECT b.id, b.title, b.classification, b.budget_type, bh.amount, bh.time_months, \
                CAST(TIMESTAMPDIFF(MONTH, ?, DATE_ADD(bh.payment_date, INTERVAL \
                    (CEIL(TIMESTAMPDIFF(MONTH, bh.payment_date, ?) / bh.time_months) \
                    * bh.time_months) MONTH)) AS UNSIGNED INT) AS months_until_payment, \
                CAST(CEIL(bh.amount / bh.time_months) AS INT) AS amount_monthly, \
                CAST(IFNULL(-1 * SUM(st.amount), 0) AS INT) AS amount_spent, \
                CAST(IFNULL(-1 * SUM(ct.amount), 0) AS INT) AS amount_contributed, \
                CAST(IFNULL(-1 * SUM(IF(st.date >= ?, st.amount, 0)), 0) AS INT) AS amount_spent_month, \
                CAST(IFNULL(-1 * SUM(IF(ct.date >= ?, ct.amount, 0)), 0) AS INT) AS amount_contributed_month \
            FROM budget b \
                JOIN users u ON u.id = b.user_id \
                JOIN budget_history bh ON bh.budget_id = b.id \
                LEFT JOIN budget_category_mapping bcm ON bcm.budget_id = b.id \
                LEFT JOIN transaction_category c ON c.id = bcm.category_id \
                LEFT JOIN bank_transaction st ON st.category_id = c.id \
                    AND st.hide_from_calculations = FALSE \
                    AND st.date >= ? \
                    AND st.date < ? \
                    AND EXISTS ( \
                        SELECT ba.id FROM bank_account ba \
                            JOIN bank_account_user bau ON bau.bank_account_id = ba.id \
                        WHERE ba.id = st.bank_account_id \
                            AND bau.user_id = u.id \
                    ) \
                LEFT JOIN bank_transaction ct ON ct.contribution_budget_id = b.id \
                    AND ct.hide_from_calculations = FALSE \
                    AND ct.date >= DATE_ADD(bh.payment_date, INTERVAL \
                        ((CEIL(TIMESTAMPDIFF(MONTH, bh.payment_date, ?) - 1) / bh.time_months) \
                        * bh.time_months) MONTH) \
                    AND ct.date < ? \
                    AND EXISTS ( \
                        SELECT ba.id FROM bank_account ba \
                            JOIN bank_account_user bau ON bau.bank_account_id = ba.id \
                        WHERE ba.id = ct.bank_account_id \
                            AND bau.user_id = u.id \
                    ) \
            WHERE u.id = ? \
                AND CEIL(TIMESTAMPDIFF(MONTH, bh.payment_date, ?) / bh.time_months) >= 0 \
                AND (bh.recur_count IS NULL OR DATE_ADD(bh.payment_date, INTERVAL (bh.time_months * bh.recur_count) MONTH) >= ?) \
            GROUP BY b.id \
            ORDER BY b.sort_order ASC \
    ";

    let (begin_date, end_date) = date_util::get_date_range(year, month)?;

    let query = sqlx::query_as::<_, BudgetDetails>(sql)
        .bind(begin_date)
        .bind(begin_date)
        .bind(begin_date)
        .bind(begin_date)
        .bind(begin_date)
        .bind(end_date)
        .bind(begin_date)
        .bind(end_date)
        .bind(user.id)
        .bind(begin_date)
        .bind(begin_date);

    let budget_details_list = query.fetch_all(conn).await?;
    Ok(budget_details_list)
}*/