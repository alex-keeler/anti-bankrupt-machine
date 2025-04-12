use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::models::bank_accounts::BankAccount;
use crate::models::data::application_error::ApplicationError;
use crate::models::DB;
use crate::models::users::User;
use crate::services::bank_service;

#[get("/api/secure/bank-account/editable/list")]
pub async fn get_editable_bank_accounts(user: User, mut conn: Connection<DB>) -> Result<Json<Vec<BankAccount>>, ApplicationError> {
    let accounts = bank_service::find_editable_accounts_for_user(user, &mut conn).await?;
    Ok(Json(accounts))
}