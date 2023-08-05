use rocket_db_pools::Connection;
use rocket::serde::json::Json;
use crate::models::DB;
use crate::models::query::transaction_details::TransactionDetails;
use crate::models::users::User;
use crate::services::bank_transaction_service;

#[post("/secure/api/set-category/<transaction_id>/<category_id>")]
pub async fn set_category(transaction_id: i64, category_id: i64, user: User, mut conn: Connection<DB>) -> Json<TransactionDetails> {
    bank_transaction_service::set_transaction_category(transaction_id, Some(category_id), &user, &mut *conn).await.unwrap();
    let transaction_details = bank_transaction_service::get_transaction_details_by_id(transaction_id, &user, &mut *conn).await.unwrap();
    Json(transaction_details)
}

#[post("/secure/api/remove-category/<transaction_id>")]
pub async fn remove_category(transaction_id: i64, user: User, mut conn: Connection<DB>) -> Json<TransactionDetails> {
    let _ = bank_transaction_service::set_transaction_category(transaction_id, None, &user, &mut *conn).await.unwrap();
    let transaction_details = bank_transaction_service::get_transaction_details_by_id(transaction_id, &user, &mut *conn).await.unwrap();
    Json(transaction_details)
}