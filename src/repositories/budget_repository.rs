use std::error::Error;
use chrono::NaiveDate;
use sqlx::MySql;
use sqlx::pool::PoolConnection;
use crate::models::budget::Budget;
use crate::models::query::budget_details::BudgetDetails;
use crate::models::transaction_categories::CategoryClassification;
use crate::models::users::User;

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
                CAST(IFNULL(-1 * SUM(st.amount), 0) AS INT) AS amount_spent, \
                CAST(IFNULL(-1 * SUM(ct.amount), 0) AS INT) AS amount_contributed \
            FROM budget b \
                JOIN users u ON u.id = b.user_id \
                JOIN budget_history bh ON bh.budget_id = b.id \
                LEFT JOIN budget_category_mapping bcm ON bcm.budget_id = b.id \
                LEFT JOIN transaction_category c ON c.id = bcm.category_id \
                LEFT JOIN bank_transaction st ON st.category_id = c.id \
                    AND st.date >= ? \
                    AND st.date < ? \
                    AND EXISTS ( \
                        SELECT ba.id FROM bank_account ba \
                            JOIN bank_account_user bau ON bau.bank_account_id = ba.id \
                        WHERE ba.id = st.bank_account_id \
                            AND bau.user_id = u.id \
                    ) \
                LEFT JOIN bank_transaction ct ON ct.contribution_budget_id = b.id \
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

    let begin_date = NaiveDate::from_ymd_opt(year, month, 1).ok_or("Cannot parse begin date")?;
    let (end_year, end_month) = match month {
        12 => (year + 1, 1),
        _ => (year, month + 1),
    };
    let end_date = NaiveDate::from_ymd_opt(end_year, end_month, 1).ok_or("Cannot parse end date")?;

    let query = sqlx::query_as::<_, BudgetDetails>(sql)
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