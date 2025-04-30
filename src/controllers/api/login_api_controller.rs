use rocket::form::Form;
use rocket::State;
use rocket_db_pools::Connection;
use crate::configuration::AppConfig;
use crate::models::data::application_error::ApplicationError;
use crate::models::data::application_error::ApplicationError::AuthenticationError;
use crate::models::DB;
use crate::services::{user_service, user_token_service};
use crate::services::user_token_service::UserClaims;

#[derive(FromForm)]
pub struct Login<'r> {
    username: &'r str,
    password: &'r str
}

#[post("/api/login", data = "<form>")]
pub async fn login(form: Form<Login<'_>>, mut conn: Connection<DB>, config: &State<AppConfig>) -> Result<String, ApplicationError> {

    let user = user_service::login(form.username, form.password, &mut conn).await?;
    if user.is_none() {
        return Err(AuthenticationError(String::from("User not found.")));
    }

    let user = user.unwrap();
    let user_jwt = user_token_service::build_user_jwt(UserClaims::from(user), &config)?;

    Ok(user_jwt)
}