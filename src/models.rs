pub mod users;
pub mod bank_accounts;
pub mod bank_account_users;
pub mod bank_transactions;
pub mod transaction_categories;

pub mod query;

use rocket_db_pools::Database;

#[derive(Database)]
#[database("budgetmanager")]
pub struct DB(sqlx::MySqlPool);