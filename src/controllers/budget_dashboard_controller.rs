use chrono::Datelike;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use crate::models::budget::SpendingType;
use crate::models::DB;
use crate::models::users::User;
use crate::services::budget_service;
use crate::util::date_util;

#[get("/secure/budget/dashboard?<year>&<month>")]
pub async fn get_dashboard(year: Option<i32>, month: Option<u32>, user: User, mut conn: Connection<DB>) -> Template {

    let current_date = chrono::Local::now();
    let year = year.unwrap_or(current_date.year());
    let month = month.unwrap_or(current_date.month());

    let budget_state_list = budget_service::find_budget_states_by_user_and_month(&user, year, month, &mut conn).await.unwrap();

    let mut variable_spent = 0;
    let mut variable_budgeted = 0;

    for budget_state in &budget_state_list {
        if matches!(budget_state.spending_type, SpendingType::Variable) {
            variable_spent += budget_state.amount_spent_this_month;
            variable_budgeted += budget_state.amount_monthly;
        }
    }

    Template::render("pages/budget-dashboard", context! { budget_state_list, year, month,
        year_range_start: current_date.year() - 2, year_range_end: current_date.year() + 2,
        month_range: date_util::get_months(), variable_spent, variable_budgeted })
}