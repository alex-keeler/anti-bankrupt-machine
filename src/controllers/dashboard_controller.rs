use rocket_dyn_templates::{context, Template};
use rocket_db_pools::Connection;
use crate::services::user_service;
use crate::models::DB;
use crate::models::users::User;

#[get("/secure/dashboard")]
pub async fn get_dashboard(_user: User, mut conn: Connection<DB>) -> Template {
    let user = user_service::find_user_by_id(1, &mut conn).await;
    Template::render("pages/dashboard", context! { user: user.unwrap() })
}