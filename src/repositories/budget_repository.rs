use std::error::Error;

use sqlx::{MySql, Row};
use sqlx::pool::PoolConnection;

use crate::models::budget::Budget;
use crate::models::query::budget_details::BudgetDetails;
use crate::models::transaction_categories::CategoryClassification;
use crate::models::users::User;
use crate::util::date_util;

pub async fn find_by_id(id: i64, conn: &mut PoolConnection<MySql>) -> Result<Budget, Box<dyn Error>> {
    let sql = "\
            SELECT b.* FROM budget b \
            WHERE b.id = ? \
    ";

    let query = sqlx::query_as::<_, Budget>(sql)
        .bind(id);

    let budget = query.fetch_one(conn).await?;

    Ok(budget)
}

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
}

pub async fn get_amount_spent_unbudgeted_for_user_and_month(user: &User, year: i32, month: u32, conn: &mut PoolConnection<MySql>) -> Result<i32, Box<dyn Error>> {
    let sql = "\
            SELECT CAST(IFNULL(-1 * SUM(bt.amount), 0) AS INT) AS amount_spent \
            FROM bank_transaction bt \
                JOIN bank_account ba ON ba.id = bt.bank_account_id \
                JOIN bank_account_user bau ON bau.bank_account_id = ba.id \
                JOIN users u ON u.id = bau.user_id \
                LEFT JOIN transaction_category tc ON tc.id = bt.category_id \
                LEFT JOIN transaction_category_group tcg ON tcg.id = tc.category_group_id \
            WHERE bt.date >= ? \
                AND bt.date < ? \
                AND bt.hide_from_calculations = FALSE \
                AND u.id = ? \
                AND NOT EXISTS ( \
                    SELECT * FROM budget_category_mapping bcm \
                        JOIN budget b ON b.id = bcm.budget_id \
                    WHERE bcm.category_id = tc.id \
                        AND b.user_id = u.id \
                ) \
                AND ((bt.category_id IS NULL AND bt.contribution_budget_id IS NULL) OR tcg.classification != ?) \
    ";

    let (begin_date, end_date) = date_util::get_date_range(year, month)?;

    let query = sqlx::query(sql)
        .bind(begin_date)
        .bind(end_date)
        .bind(user.id)
        .bind(CategoryClassification::Income);

    let row = query.fetch_one(conn).await?;
    Ok(row.get("amount_spent"))
}

pub async fn find_all_by_classification(classification: CategoryClassification, user: &User, conn: &mut PoolConnection<MySql>) -> Result<Vec<Budget>, Box<dyn Error>> {
    let sql = "\
            SELECT b.* FROM budget b
                JOIN users u on u.id = b.user_id
            WHERE u.id = ?
                AND b.classification = ?
    ";

    let query = sqlx::query_as::<_, Budget>(sql)
        .bind(user.id)
        .bind(classification);

    let budget_list = query.fetch_all(conn).await?;
    Ok(budget_list)
}

pub async fn find_all_financeable(user: &User, conn: &mut PoolConnection<MySql>) -> Result<Vec<Budget>, Box<dyn Error>> {
    let sql = "\
            SELECT b.* FROM budget b
                JOIN users u on u.id = b.user_id
                JOIN budget_history bh on bh.budget_id = b.id
                    AND NOT EXISTS (
                        SELECT bh2.id FROM budget_history bh2
                        WHERE bh2.budget_id = b.id
                            AND bh2.payment_date > bh.payment_date
                    )
            WHERE u.id = ?
                AND b.classification IN (?, ?)
                AND bh.time_months > 1
    ";

    let query = sqlx::query_as::<_, Budget>(sql)
        .bind(user.id)
        .bind(CategoryClassification::Needs)
        .bind(CategoryClassification::Wants);

    let budget_list = query.fetch_all(conn).await?;
    Ok(budget_list)
}