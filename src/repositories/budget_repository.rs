use std::error::Error;
use chrono::NaiveDate;
use sqlx::MySql;
use sqlx::pool::PoolConnection;
use crate::models::budget::Budget;
use crate::models::query::budget_details::BudgetDetails;
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

    let mut sql = "\
            SELECT b.id, b.title, b.amount, b.time_months, \
                IF(b.start_date <= ?,\
                    ((? * 12 + ?) - (YEAR(b.start_date) * 12 + MONTH(b.start_date))) % b.time_months = 0, \
                    FALSE) AS on_payment_month, \
                b.budget_type, b.classification, CAST(IFNULL(-1 * SUM(bt.amount), 0) AS INT) AS amount_spent \
            FROM budget b \
                JOIN budget_category_mapping bcm ON bcm.budget_id = b.id \
                JOIN transaction_category c ON c.id = bcm.category_id \
                LEFT JOIN bank_transaction bt ON bt.category_id = c.id \
                    AND EXISTS (\
                        SELECT ba.id FROM bank_account ba \
                            JOIN bank_account_user bau ON bau.bank_account_id = ba.id \
                            JOIN users u ON u.id = bau.user_id \
                        WHERE ba.id = bt.bank_account_id
                            AND u.id = ? \
                    ) \
                    AND bt.date >= ? \
                    AND bt.date < ? \
            GROUP BY b.id \
            ORDER BY b.id ASC \
    ";

    let begin_date = NaiveDate::from_ymd_opt(year, month, 1).ok_or("Cannot parse begin date")?;
    let (end_year, end_month) = match month {
        12 => (year + 1, 1),
        _ => (year, month + 1),
    };
    let end_date = NaiveDate::from_ymd_opt(end_year, end_month, 1).ok_or("Cannot parse end date")?;

    let query = sqlx::query_as::<_, BudgetDetails>(sql)
        .bind(begin_date)
        .bind(year)
        .bind(month)
        .bind(user.id)
        .bind(begin_date)
        .bind(end_date);

    let budget_details_list = query.fetch_all(conn).await?;
    Ok(budget_details_list)
}