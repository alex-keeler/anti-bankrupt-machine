use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::models::data::application_error::ApplicationError;
use crate::models::query::budgets_by_classification::BudgetsByClassification;
use crate::models::DB;
use crate::models::query::budget_details::BudgetDetails;
use crate::models::query::budget_state::BudgetState;
use crate::models::users::User;
use crate::services::budget_service;

#[get("/api/secure/budget/state/list?<year>&<month>")]
pub async fn get_budget_states(year: i32, month: u32, user: User, mut conn: Connection<DB>) -> Result<Json<Vec<BudgetState>>, ApplicationError> {
    let budget_state_list = budget_service::find_budget_states_by_user_and_month(&user, year, month, &mut conn).await?;
    Ok(Json(budget_state_list))
}

#[get("/api/secure/budget/details/list?<year>&<month>")]
pub async fn get_budget_details(year: i32, month: u32, user: User, mut conn: Connection<DB>) -> Result<Json<Vec<BudgetDetails>>, ApplicationError> {
    let budget_details_list = budget_service::find_budget_details_by_user_and_month(&user, year, month, &mut conn).await?;
    Ok(Json(budget_details_list))
}

#[get("/api/secure/budgets/group-by-classification/list")]
pub async fn get_budgets_grouped_by_classification(user: User, mut conn: Connection<DB>) -> Result<Json<Vec<BudgetsByClassification>>, ApplicationError> {
    // TODO: shouldn't this return budgets applicable for a given month?
    let budgets_by_classification = budget_service::find_budgets_by_user_grouped_by_classification(&user, &mut conn).await?;
    Ok(Json(budgets_by_classification))
}