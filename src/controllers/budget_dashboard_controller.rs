use chrono::Datelike;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use crate::models::budget::BudgetType;
use crate::models::DB;
use crate::models::users::User;
use crate::services::budget_service;

const MONTH_RANGE: [(&str, i32); 12] = [
    ("January", 1), ("February", 2), ("March", 3), ("April", 4), ("May", 5), ("June", 6),
    ("July", 7), ("August", 8), ("September", 9), ("October", 10), ("November", 11), ("December", 12)
];

#[get("/secure/budget/dashboard?<year>&<month>")]
pub async fn get_dashboard(year: Option<i32>, month: Option<u32>, user: User, mut conn: Connection<DB>) -> Template {

    let current_date = chrono::Local::now();
    let year = year.unwrap_or(current_date.year());
    let month = month.unwrap_or(current_date.month());

    let budget_details_list = budget_service::get_budget_details_list_by_user_and_month(&user, year, month, &mut *conn).await.unwrap();

    let mut variable_spent = 0;
    let mut variable_budgeted = 0;
    for budget in &budget_details_list {
        if matches!(budget.budget_type, BudgetType::Variable) && budget.months_until_payment == 0 {
            variable_spent += budget.amount_spent;
            variable_budgeted += budget.amount;
        }
    }
    
    Template::render("pages/budget-dashboard", context! { budget_details_list, year, month,
        current_year: current_date.year(), month_range: MONTH_RANGE, variable_spent, variable_budgeted })
}