use std::error::Error;
use chrono::NaiveDate;
use sqlx::MySql;
use sqlx::pool::PoolConnection;
use crate::models::query::budget_details::BudgetDetails;
use crate::models::users::User;
use crate::repositories::budget_repository;

pub async fn get_budget_details_list_by_user_and_month(user: &User, year: i32, month: u32, conn: &mut PoolConnection<MySql>) -> Result<Vec<BudgetDetails>, Box<dyn Error>> {
    budget_repository::get_details_list_for_user_and_month(user, year, month, conn).await
}