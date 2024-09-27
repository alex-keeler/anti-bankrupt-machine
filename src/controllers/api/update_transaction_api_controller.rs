use rocket_db_pools::Connection;
use rocket::serde::json::Json;
use crate::models::DB;
use crate::models::query::transaction_details::TransactionDetails;
use crate::models::users::User;
use crate::services::bank_transaction_service;

#[post("/secure/api/transaction/set-user-description/<transaction_id>", data="<user_description>")]
pub async fn set_user_description(transaction_id: i64, mut user_description: Option<String>, user: User, mut conn: Connection<DB>) -> Json<TransactionDetails> {

    if user_description.is_some() && user_description.as_ref().unwrap().is_empty() {
        user_description = None;
    }

    bank_transaction_service::set_transaction_user_description(transaction_id, user_description, &user, &mut conn).await.unwrap();
    let transaction_details = bank_transaction_service::get_transaction_details_by_id(transaction_id, &user, &mut conn).await.unwrap();
    Json(transaction_details)
}

#[post("/secure/api/transaction/set-comment/<transaction_id>", data="<comment>")]
pub async fn set_comment(transaction_id: i64, mut comment: Option<String>, user: User, mut conn: Connection<DB>) -> Json<TransactionDetails> {

    if comment.is_some() && comment.as_ref().unwrap().is_empty() {
        comment = None;
    }

    bank_transaction_service::set_transaction_comment(transaction_id, comment, &user, &mut conn).await.unwrap();
    let transaction_details = bank_transaction_service::get_transaction_details_by_id(transaction_id, &user, &mut conn).await.unwrap();
    Json(transaction_details)
}
