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
    bank_transaction_service::set_transaction_category(transaction_id, None, &user, &mut *conn).await.unwrap();
    let transaction_details = bank_transaction_service::get_transaction_details_by_id(transaction_id, &user, &mut *conn).await.unwrap();
    Json(transaction_details)
}

#[post("/secure/api/set-contribution-budget/<transaction_id>/<budget_id>")]
pub async fn set_contribution_budget(transaction_id: i64, budget_id: i64, user: User, mut conn: Connection<DB>) -> Json<TransactionDetails> {
    bank_transaction_service::set_transaction_contribution_budget(transaction_id, Some(budget_id), &user, &mut *conn).await.unwrap();
    let transaction_details = bank_transaction_service::get_transaction_details_by_id(transaction_id, &user, &mut *conn).await.unwrap();
    Json(transaction_details)
}

#[post("/secure/api/remove-contribution-budget/<transaction_id>")]
pub async fn remove_contribution_budget(transaction_id: i64, user: User, mut conn: Connection<DB>) -> Json<TransactionDetails> {
    bank_transaction_service::set_transaction_contribution_budget(transaction_id, None, &user, &mut *conn).await.unwrap();
    let transaction_details = bank_transaction_service::get_transaction_details_by_id(transaction_id, &user, &mut *conn).await.unwrap();
    Json(transaction_details)
}

#[post("/secure/api/set-user-description/<transaction_id>", data="<user_description>")]
pub async fn set_user_description(transaction_id: i64, mut user_description: Option<String>, user: User, mut conn: Connection<DB>) -> Json<TransactionDetails> {

    if user_description.is_some() && user_description.as_ref().unwrap().is_empty() {
        user_description = None;
    }

    bank_transaction_service::set_transaction_user_description(transaction_id, user_description, &user, &mut *conn).await.unwrap();
    let transaction_details = bank_transaction_service::get_transaction_details_by_id(transaction_id, &user, &mut *conn).await.unwrap();
    Json(transaction_details)
}

#[post("/secure/api/set-comment/<transaction_id>", data="<comment>")]
pub async fn set_comment(transaction_id: i64, mut comment: Option<String>, user: User, mut conn: Connection<DB>) -> Json<TransactionDetails> {

    if comment.is_some() && comment.as_ref().unwrap().is_empty() {
        comment = None;
    }

    bank_transaction_service::set_transaction_comment(transaction_id, comment, &user, &mut *conn).await.unwrap();
    let transaction_details = bank_transaction_service::get_transaction_details_by_id(transaction_id, &user, &mut *conn).await.unwrap();
    Json(transaction_details)
}

#[post("/secure/api/set-hide-from-calculations/<transaction_id>/<hide_from_calculations>")]
pub async fn set_hide_from_calculations(transaction_id: i64, hide_from_calculations: bool, user: User, mut conn: Connection<DB>) -> Json<TransactionDetails> {

    bank_transaction_service::set_transaction_hide_from_calculations(transaction_id, hide_from_calculations, &user, &mut *conn).await.unwrap();
    let transaction_details = bank_transaction_service::get_transaction_details_by_id(transaction_id, &user, &mut *conn).await.unwrap();
    Json(transaction_details)
}
