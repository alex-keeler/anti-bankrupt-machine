#[macro_use]
extern crate rocket;

use std::path::{Path, PathBuf};

use rocket::{Build, fairing, Rocket};
use rocket::fairing::AdHoc;
use rocket::fs::NamedFile;
use rocket::http::Method;
use rocket::response::Redirect;
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;
use rocket_cors::{AllowedOrigins, CorsOptions};

use crate::models::DB;
use crate::models::users::User;

mod configuration;
mod controllers;
mod services;
mod repositories;
mod models;
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

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::some_exact(&["http://localhost:5173"]))
        .allowed_methods(
            vec![Method::Get, Method::Post]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    // TODO: need to find with a better way to mount controller paths
    rocket::build()
        .mount("/", routes![
            index,
            // controllers::dashboard_controller::get_dashboard,
            // controllers::login_controller::get_login_form,
            // controllers::login_controller::login_post,
            // controllers::login_controller::logout,
            // controllers::bank_record_upload_controller::get_upload_form,
            // controllers::bank_record_upload_controller::upload_post,
            // controllers::transaction_dashboard_controller::get_dashboard,
            // controllers::budget_dashboard_controller::get_dashboard,
            // controllers::cash_flow_dashboard_controller::get_dashboard,
            controllers::api::login_api_controller::login,
            controllers::api::budget_api_controller::get_budget_states,
            controllers::api::budget_api_controller::get_budget_details,
            controllers::api::budget_api_controller::get_budgets_grouped_by_classification,
            controllers::api::update_transaction_api_controller::set_user_description,
            controllers::api::update_transaction_api_controller::set_comment,
            controllers::api::cash_flow_api_controller::get_cash_flow_data,
            controllers::api::transaction_group_api_controller::get_top_transaction_groups_by_category,
            controllers::api::update_budget_transaction_mapping_api_controller::add_budget_transaction_mapping,
            controllers::api::update_budget_transaction_mapping_api_controller::remove_budget_transaction_mapping,
            controllers::api::update_budget_transaction_mapping_api_controller::update_budget_transaction_mapping,
            controllers::api::transaction_api_controller::get_transaction_list,
            controllers::api::transaction_api_controller::get_top_transactions_by_budget,
            controllers::api::transaction_api_controller::assign_budget_transaction_mappings,
            controllers::api::bank_account_api_controller::get_editable_bank_accounts,
            controllers::api::bank_record_api_controller::upload_post,
        ])
        .mount("/static", routes![static_files])
        .register("/", catchers![
            //controllers::catchers::unauthorized_catcher::redirect_or_access_denied,
        ])
        .attach(AdHoc::config::<configuration::AppConfig>())
        .attach(cors.to_cors().unwrap())
        .attach(DB::init())
        .attach(AdHoc::try_on_ignite("DB Migrations", run_migrations))
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    println!("Running migrations...");
    match DB::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
            Ok(_) => {
                println!("Successfully ran migrations");
                Ok(rocket)
            }
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        }
        None => Err(rocket),
    }
}
