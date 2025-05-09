use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::models::data::application_error::ApplicationError;
use crate::models::DB;
use crate::models::users::User;
use crate::models::data::spending_report::SpendingReport;
use crate::services::bank_transaction_service;

#[get("/api/secure/spending/breakdown/by-month?<year>&<month>")]
pub async fn get_month_spending_breakdown(year: i32, month: u32, user: User, mut conn: Connection<DB>) -> Result<Json<SpendingReport>, ApplicationError> {
    let spending_report = bank_transaction_service::build_month_spending_report(year, month, &user, &mut conn).await?;
    Ok(Json(spending_report))
}