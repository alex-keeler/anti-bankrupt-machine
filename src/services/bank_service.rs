use std::error::Error;
use rocket_db_pools::Connection;
use sqlx::MySql;
use sqlx::pool::PoolConnection;
use crate::models::bank_account_users::BankAccountUserRole;
use crate::models::bank_accounts::BankAccount;
use crate::models::DB;
use crate::models::users::User;
use crate::repositories::bank_account_repository;

pub async fn find_editable_accounts_for_user(user: User, conn: Connection<DB>) -> Result<Vec<BankAccount>, Box<dyn Error>> {
    bank_account_repository::find_accounts_for_user(user, Some(vec![BankAccountUserRole::Owner, BankAccountUserRole::Editor]), conn).await
}

pub async fn find_account_by_id(id: i64, conn: &mut PoolConnection<MySql>) -> Result<BankAccount, Box<dyn Error>> {
    bank_account_repository::find_account_by_id(id, conn).await
}