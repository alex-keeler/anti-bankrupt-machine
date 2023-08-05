use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use crate::models::DB;
use crate::models::users::User;
use crate::services::bank_transaction_service;

#[get("/secure/transaction/dashboard")]
pub async fn get_dashboard(user: User, mut conn: Connection<DB>) -> Template {

    let transaction_details_list = bank_transaction_service::get_transaction_details_by_user(user, &mut *conn).await.unwrap();
    let category_group_list = bank_transaction_service::get_transaction_category_group_select_list(&mut *conn).await.unwrap();

    Template::render("pages/transaction-dashboard", context! { transaction_details_list, category_group_list })
}