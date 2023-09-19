use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use crate::models::DB;
use crate::models::transaction_categories::CategoryClassification;
use crate::models::users::User;
use crate::services::{bank_transaction_service, budget_service};

#[get("/secure/transaction/dashboard")]
pub async fn get_dashboard(user: User, mut conn: Connection<DB>) -> Template {

    let transaction_details_list = bank_transaction_service::get_transaction_details_by_user(&user, &mut *conn).await.unwrap();
    let category_group_list = bank_transaction_service::get_transaction_category_group_select_list(&user, &mut *conn).await.unwrap();
    let saving_budgets = budget_service::find_all_budgets_by_classification(CategoryClassification::Savings, &user, &mut *conn).await.unwrap();
    let financeable_budgets = budget_service::find_all_financeable_budgets(&user, &mut *conn).await.unwrap();

    Template::render("pages/transaction-dashboard", context! {
        transaction_details_list, category_group_list, saving_budgets, financeable_budgets
    })
}