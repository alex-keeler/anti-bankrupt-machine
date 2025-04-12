use rocket_db_pools::Connection;
use rocket::serde::json::Json;
use crate::models::budget_transaction_mapping::TransactionBehavior;
use crate::models::DB;
use crate::models::query::budget_transaction_mapping_details::BudgetTransactionMappingDetails;
use crate::models::users::User;
use crate::services::bank_transaction_service;

#[deprecated] // replaced by transaction_api_controller::assign_budget_transaction_mappings
#[post("/secure/api/budget-transaction-mapping/add/<transaction_id>/<budget_id>?<amount>&<behavior>")]
pub async fn add_budget_transaction_mapping(
    transaction_id: i64, budget_id: i64, user: User, mut conn: Connection<DB>, amount: i32,
    behavior: TransactionBehavior
) -> Json<BudgetTransactionMappingDetails> {

    let mapping_id = bank_transaction_service::add_budget_transaction_mapping(transaction_id, budget_id, &user, amount, behavior, &mut conn).await.unwrap();
    let mapping_details = bank_transaction_service::get_budget_transaction_mapping_details_by_id(mapping_id, &user, &mut conn).await.unwrap();
    Json(mapping_details)
}

#[deprecated] // replaced by transaction_api_controller::assign_budget_transaction_mappings
#[post("/secure/api/budget-transaction-mapping/remove/<budget_transaction_mapping_id>")]
pub async fn remove_budget_transaction_mapping(
    budget_transaction_mapping_id: i64, user: User, mut conn: Connection<DB>
) {
    bank_transaction_service::remove_budget_transaction_mapping(budget_transaction_mapping_id, &user, &mut conn).await.unwrap();
}

#[deprecated] // replaced by transaction_api_controller::assign_budget_transaction_mappings
#[post("/secure/api/budget-transaction-mapping/update/<budget_transaction_mapping_id>?<amount>&<behavior>")]
pub async fn update_budget_transaction_mapping(
    budget_transaction_mapping_id: i64, amount: i32, behavior: TransactionBehavior, user: User, mut conn: Connection<DB>
) -> Json<BudgetTransactionMappingDetails> {

    bank_transaction_service::update_budget_transaction_mapping(budget_transaction_mapping_id, &user, amount, behavior, &mut conn).await.unwrap();
    let mapping_details = bank_transaction_service::get_budget_transaction_mapping_details_by_id(budget_transaction_mapping_id, &user, &mut conn).await.unwrap();
    Json(mapping_details)
}
