use chrono::NaiveDate;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::models::DB;
use crate::models::query::transaction_group::TransactionGroup;
use crate::models::users::User;
use crate::services::bank_transaction_service;

#[get("/secure/api/transaction-group/top-by-budget/<budget_id>/<limit>/<year>/<month>")]
pub async fn get_top_transaction_groups_by_category(budget_id: i64, limit: i32, year: i32, month: u32, user: User, mut conn: Connection<DB>) -> Json<Vec<TransactionGroup>> {

    let begin_date = NaiveDate::from_ymd_opt(year, month, 1).ok_or("Cannot parse begin date").unwrap();
    let (end_year, end_month) = match month {
        12 => (year + 1, 1),
        _ => (year, month + 1),
    };
    let end_date = NaiveDate::from_ymd_opt(end_year, end_month, 1).ok_or("Cannot parse end date").unwrap();

    let transaction_groups = bank_transaction_service::get_top_transaction_groups_by_budget_id(
        budget_id, limit, Some(begin_date), Some(end_date), &user, &mut *conn).await.unwrap();

    Json(transaction_groups)
}