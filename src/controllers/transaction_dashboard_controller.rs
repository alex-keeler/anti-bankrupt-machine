use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use crate::models::DB;
use crate::models::users::User;
use crate::services::{bank_transaction_service, budget_service};

#[get("/secure/transaction/dashboard")]
pub async fn get_dashboard(user: User, mut conn: Connection<DB>) -> Template {

    let transaction_details_list = bank_transaction_service::get_full_transaction_details_by_user(&user, &mut *conn).await.unwrap();
    let budgets_by_classification_list = budget_service::find_budgets_by_user_grouped_by_classification(&user, &mut *conn).await.unwrap();

    Template::render("pages/transaction-dashboard", context! {
        transaction_details_list, budgets_by_classification_list,
    })
}