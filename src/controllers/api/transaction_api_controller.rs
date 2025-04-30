use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::models::budget_transaction_mapping::BudgetTransactionMapping;
use crate::models::data::application_error::ApplicationError;
use crate::models::query::full_transaction_details::FullTransactionDetails;
use crate::models::DB;
use crate::models::query::budget_transaction_mapping_details::BudgetTransactionMappingDetails;
use crate::models::query::transaction_details::TransactionDetails;
use crate::models::users::User;
use crate::services::bank_transaction_service;

#[get("/api/secure/transaction/list?<year>&<month>")]
pub async fn get_transaction_list(year: i32, month: u32, user: User, mut conn: Connection<DB>) -> Result<Json<Vec<FullTransactionDetails>>, ApplicationError> {
    let transaction_details_list = bank_transaction_service::get_full_transaction_details_by_user_and_month(&user, year, month, &mut conn).await?;
    Ok(Json(transaction_details_list))
}

#[get("/api/secure/transaction/top/by-budget/<budget_id>?<year>&<month>&<limit>")]
pub async fn get_top_transactions_by_budget(budget_id: i64, year: i32, month: u32, limit: Option<i32>, user: User, mut conn: Connection<DB>) -> Result<Json<Vec<TransactionDetails>>, ApplicationError> {
    let transaction_details_list = bank_transaction_service::get_top_transactions_by_budget_id_and_month(budget_id, year, month, limit, &user, &mut conn).await?;
    Ok(Json(transaction_details_list))
}

#[post("/api/secure/transaction/<transaction_id>/budget-mappings/assign", data = "<mappings>")]
pub async fn assign_budget_transaction_mappings(transaction_id: i64, mappings: Json<Vec<BudgetTransactionMapping>>, user: User, mut conn: Connection<DB>) -> Result<Json<Vec<BudgetTransactionMappingDetails>>, ApplicationError> {
    bank_transaction_service::assign_budget_transaction_mappings(transaction_id, mappings.0, &user, &mut conn).await?;
    let mapping_details_list = bank_transaction_service::get_budget_transaction_mapping_details_by_transaction_id(transaction_id, &user, &mut conn).await?;
    Ok(Json(mapping_details_list))
}