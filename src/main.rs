#[macro_use] extern crate rocket;

use std::path::{Path, PathBuf};
use rocket::{Build, Rocket};
use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use rocket_db_pools::Database;
use crate::models::users::User;

mod controllers;
mod services;
mod repositories;
mod models;
mod data;
mod util;
mod tests;
mod extensions;

#[get("/")]
fn index(user: Option<User>) -> Redirect {
    match user {
        Some(_) => Redirect::to(uri!(controllers::dashboard_controller::get_dashboard)),
        None => Redirect::to(uri!(controllers::login_controller::get_login_form)),
    }
}

#[get("/<file..>")]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[launch]
fn rocket() -> Rocket<Build> {
    // TODO: need to find with a better way to mount controller paths
    rocket::build()
        .mount("/", routes![
            index,
            controllers::dashboard_controller::get_dashboard,
            controllers::login_controller::get_login_form,
            controllers::login_controller::login_post,
            controllers::login_controller::logout,
            controllers::bank_record_upload_controller::get_upload_form,
            controllers::bank_record_upload_controller::upload_post,
            controllers::transaction_dashboard_controller::get_dashboard,
            controllers::api::update_transaction_api_controller::set_category,
            controllers::api::update_transaction_api_controller::remove_category,
            controllers::api::update_transaction_api_controller::set_user_description,
            controllers::api::update_transaction_api_controller::set_comment,
        ])
        .mount("/static", routes![static_files])
        .attach(Template::custom(|engines| {
            engines.tera.register_function("format_amount", extensions::format_amount_extension::build())
        }))
        .attach(models::DB::init())
}
