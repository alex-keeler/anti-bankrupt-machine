use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::response::Redirect;
use rocket_db_pools::Connection;
use crate::models::data::application_error::ApplicationError;
use crate::models::DB;
use crate::models::users::User;
use crate::services::{bank_service, bank_transaction_service};

#[derive(FromForm)]
pub struct Upload<'r> {
    bank_account_id: i64,
    file: TempFile<'r>
}

#[post("/api/secure/bank-record/upload", data = "<form>")]
pub async fn upload_post(form: Form<Upload<'_>>, user: User, mut conn: Connection<DB>) -> Result<(), ApplicationError> {
    let bank_account = bank_service::find_account_by_id(form.bank_account_id, &mut conn).await?;
    bank_transaction_service::import_bank_record_file(&form.file, user, bank_account, &mut conn).await?;
    Ok(())
}