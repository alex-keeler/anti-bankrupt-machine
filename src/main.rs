#[macro_use]
extern crate rocket;

use rocket::{Build, fairing, Rocket, Config};
use rocket::fairing::AdHoc;
use rocket::http::Method;
use rocket_db_pools::Database;
use rocket_cors::{AllowedOrigins, CorsOptions};
use configuration::AppConfig;

use crate::models::DB;

mod configuration;
mod controllers;
mod services;
mod repositories;
mod models;
mod util;
mod tests;

#[launch]
fn rocket() -> Rocket<Build> {

    // let config = Config::figment();
    // let allowed_origin = config.find_value("allowed_origin").unwrap();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::some_regex(&[r"(192\.168\..*)", r"(localhost.*)", r"(127\.0\.0\.1.*)"]))
        .allowed_methods(
            vec![Method::Get, Method::Post]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::build()
        .mount("/", routes![
            controllers::api::status_api_controller::report_status,
            controllers::api::login_api_controller::login,
            controllers::api::budget_api_controller::get_budget_states,
            controllers::api::budget_api_controller::get_budget_details,
            controllers::api::budget_api_controller::get_budgets_grouped_by_classification,
            controllers::api::update_transaction_api_controller::set_user_description,
            controllers::api::update_transaction_api_controller::set_comment,
            controllers::api::cash_flow_api_controller::get_cash_flow_data,
            controllers::api::transaction_api_controller::get_transaction_list,
            controllers::api::transaction_api_controller::get_top_transactions_by_budget,
            controllers::api::transaction_api_controller::assign_budget_transaction_mappings,
            controllers::api::bank_account_api_controller::get_editable_bank_accounts,
            controllers::api::bank_record_api_controller::upload_post,
        ])
        .attach(AdHoc::config::<AppConfig>())
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
