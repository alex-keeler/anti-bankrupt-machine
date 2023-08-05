use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::response::Redirect;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use crate::models::DB;
use crate::models::users::User;
use crate::services::{bank_service, bank_transaction_service};
use crate::controllers::dashboard_controller;

#[derive(FromForm)]
pub struct Upload<'r> {
    bank_account_id: i64,
    file: TempFile<'r>
}

#[get("/secure/upload/bank-record")]
pub async fn get_upload_form(user: User, conn: Connection<DB>) -> Template {
    let editable_accounts = bank_service::find_editable_accounts_for_user(user, conn).await.unwrap();
    Template::render("pages/bank-record-upload", context! { accounts: editable_accounts })
}

#[post("/secure/upload/bank-record", data = "<form>")]
pub async fn upload_post(form: Form<Upload<'_>>, user: User, mut conn: Connection<DB>) -> Redirect {
    let bank_account = bank_service::find_account_by_id(form.bank_account_id, &mut *conn).await.unwrap();
    bank_transaction_service::import_bank_record_file(&form.file, user, bank_account, &mut *conn).await.unwrap();
    Redirect::to(uri!(dashboard_controller::get_dashboard)) // TODO: redirect to transactions screen
}