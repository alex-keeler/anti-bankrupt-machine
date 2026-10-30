use rocket::form::Form;
use rocket::fs::TempFile;
use rocket_db_pools::Connection;
use tempfile::NamedTempFile;
use crate::models::bank_account_users::BankAccountUserRole;
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
pub async fn upload_post(mut form: Form<Upload<'_>>, user: User, mut conn: Connection<DB>) -> Result<(), ApplicationError> {

    let temp_file = NamedTempFile::new().or(Err("Could not create a temp file"))?;

    form.file.persist_to(temp_file.path()).await.or(Err("Could not persist temp file"))?;

    let bank_account = bank_service::find_account_by_id(form.bank_account_id, &user, Some(vec![BankAccountUserRole::Editor]), &mut conn).await?;

    bank_transaction_service::import_bank_record_file(temp_file.path(), form.file.name().ok_or("Could not get filename")?, form.file.content_type().ok_or("No content type found")?, &user, bank_account, &mut conn).await?;

    Ok(())
}