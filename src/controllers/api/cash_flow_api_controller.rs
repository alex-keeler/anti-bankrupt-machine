use std::cmp::Reverse;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use serde::{Serialize, Deserialize};
use crate::models::DB;
use crate::models::query::budget_state::BudgetState;
use crate::models::users::User;
use crate::services::{bank_transaction_service, budget_service};

#[get("/secure/api/cash-flow/data/<year>/<month>")]
pub async fn get_cash_flow_data(year: i32, month: u32, user: User, mut conn: Connection<DB>) -> Json<CashFlowData> {

    // let mut income_details_list =
    //     bank_transaction_service::get_transaction_category_details_list_by_classification_and_month(
    //     CategoryClassification::Income, year, month, &user, &mut *conn).await.unwrap();

    // income_details_list.sort_by_key(|income| Reverse(income.total_amount));

    let mut budget_state_list =
        budget_service::find_budget_states_by_user_and_month(&user, year, month, &mut *conn)
            .await.unwrap();

    budget_state_list.sort_by_key(|budget| Reverse(budget.amount_monthly));

    let amount_spent_unbudgeted =
        bank_transaction_service::get_amount_spent_unbudgeted_for_user_and_month(&user, year, month, &mut *conn)
            .await.unwrap();

    let data = CashFlowData {
        // income_details_list,
        budget_state_list,
        amount_spent_unbudgeted,
    };
    Json(data)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CashFlowData {
    // income_details_list: Vec<TransactionCategoryDetails>,
    budget_state_list: Vec<BudgetState>,
    amount_spent_unbudgeted: i32,
}