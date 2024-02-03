pub mod users;
pub mod bank_accounts;
pub mod bank_account_users;
pub mod bank_transactions;

pub mod query;
pub mod budget;
pub mod budget_transaction_mapping;
pub mod data;

use rocket_db_pools::Database;

#[derive(Database)]
#[database("budgetmanager")]
pub struct DB(sqlx::MySqlPool);
