use rocket::form::Form;
use rocket::uri;
use rocket::http::{Cookie, CookieJar};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use crate::controllers::dashboard_controller;
use crate::models::DB;
use crate::models::users::User;
use crate::services::user_service;

#[derive(FromForm)]
pub struct Login<'r> {
    username: &'r str,
    password: &'r str
}

#[get("/login")]
pub async fn get_login_form(flash: Option<FlashMessage<'_>>) -> Template {
    if let Some(message) = &flash {
        println!("Got flash: {} {}", message.kind(), message.message());
    }
    Template::render("pages/login", context! { flash })
}

#[post("/login", data = "<form>")]
pub async fn login_post(form: Form<Login<'_>>, conn: Connection<DB>, cookie_jar: &CookieJar<'_>) -> Result<Redirect, Flash<Redirect>> {

    let user = user_service::login(form.username, form.password, conn).await;

    if user.is_err() {
        return Err(Flash::error(Redirect::to(uri!(get_login_form)), "An error has occurred"));
    }

    let user = user.unwrap();
    if user.is_none() {
        println!("Invalid username/password combination");
        return Err(Flash::error(Redirect::to(uri!(get_login_form)), "Invalid username/password combination"));
    }

    let user = user.unwrap();
    cookie_jar.add_private(Cookie::new("user_id", user.id.unwrap().to_string()));

    Ok(Redirect::to(uri!(dashboard_controller::get_dashboard)))
}

#[get("/logout")]
pub async fn logout(_user: User, cookie_jar: &CookieJar<'_>) -> Flash<Redirect> {
    cookie_jar.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to(uri!(get_login_form)), "Successfully logged out")
}