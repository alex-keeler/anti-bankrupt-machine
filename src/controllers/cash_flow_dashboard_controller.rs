use chrono::Datelike;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use crate::models::DB;
use crate::models::users::User;
use crate::util::date_util;

#[get("/secure/cash-flow/dashboard?<year>&<month>")]
pub async fn get_dashboard(year: Option<i32>, month: Option<u32>, _user: User) -> Template {

    let current_date = chrono::Local::now();
    let year = year.unwrap_or(current_date.year());
    let month = month.unwrap_or(current_date.month());

    Template::render("pages/cash-flow-dashboard", context! { year, month,
            year_range_start: current_date.year() - 2, year_range_end: current_date.year() + 2,
            month_range: date_util::get_months()})
}