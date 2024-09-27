use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::models::data::application_error::ApplicationError;
use crate::models::DB;
use crate::models::query::budget_state::BudgetState;
use crate::models::users::User;
use crate::services::budget_service;

#[get("/api/budget/state/list?<year>&<month>")]
pub async fn get_budget_list(year: i32, month: u32, user: User, mut conn: Connection<DB>) -> Result<Json<Vec<BudgetState>>, ApplicationError> {
    let budget_state_list = budget_service::find_budget_states_by_user_and_month(&user, year, month, &mut conn).await?;
    Ok(Json(budget_state_list))
}