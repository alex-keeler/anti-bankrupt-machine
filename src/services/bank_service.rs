use rocket_db_pools::Connection;
use crate::models::bank_account_users::BankAccountUserRole;
use crate::models::bank_accounts::BankAccount;
use crate::models::data::application_error::ApplicationError;
use crate::models::DB;
use crate::models::users::User;
use crate::repositories::bank_account_repository;

pub async fn find_editable_accounts_for_user(user: User, conn: &mut Connection<DB>) -> Result<Vec<BankAccount>, ApplicationError> {
    let accounts = bank_account_repository::find_accounts_for_user(user, Some(vec![BankAccountUserRole::Owner, BankAccountUserRole::Editor]), conn).await?;
    Ok(accounts)
}

pub async fn find_account_by_id(id: i64, user: &User, restrict_to_roles: Option<Vec<BankAccountUserRole>>, conn: &mut Connection<DB>) -> Result<BankAccount, ApplicationError> {
    let account = bank_account_repository::find_account_by_id(id, user, restrict_to_roles, conn).await?;
    Ok(account)
}