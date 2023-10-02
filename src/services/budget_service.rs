use std::error::Error;
use chrono::NaiveDate;
use sqlx::MySql;
use sqlx::pool::PoolConnection;
use crate::models::budget::Budget;
use crate::models::query::budget_details::BudgetDetails;
use crate::models::transaction_categories::CategoryClassification;
use crate::models::users::User;
use crate::repositories::budget_repository;

pub async fn get_budget_details_list_by_user_and_month(user: &User, year: i32, month: u32, conn: &mut PoolConnection<MySql>) -> Result<Vec<BudgetDetails>, Box<dyn Error>> {
    budget_repository::get_details_list_for_user_and_month(user, year, month, conn).await
}

pub async fn find_all_budgets_by_classification(classification: CategoryClassification, user: &User, conn: &mut PoolConnection<MySql>) -> Result<Vec<Budget>, Box<dyn Error>> {
    budget_repository::find_all_by_classification(classification, user, conn).await
}

pub async fn find_all_financeable_budgets(user: &User, conn: &mut PoolConnection<MySql>) -> Result<Vec<Budget>, Box<dyn Error>> {
    budget_repository::find_all_financeable(user, conn).await
}

pub async fn get_amount_spent_unbudgeted_for_user_and_month(user: &User, year: i32, month: u32, conn: &mut PoolConnection<MySql>) -> Result<i32, Box<dyn Error>> {
    budget_repository::get_amount_spent_unbudgeted_for_user_and_month(user, year, month, conn).await
}