use rocket_db_pools::Connection;
use crate::models::{DB, users::User};
use crate::models::data::application_error::ApplicationError;
use crate::repositories::user_repository;
use crate::services::password_hash_util;

pub async fn find_user_by_id(id: i64, conn: Connection<DB>) -> Result<User, ApplicationError> {
    let user = user_repository::find_user_by_id(id, conn).await?;
    Ok(user)
}

pub async fn login(username: &str, password: &str, conn: Connection<DB>) -> Result<Option<User>, ApplicationError> {
    let user = user_repository::find_user_by_username(username, conn).await?;

    if user.is_none() {
        return Ok(None);
    }

    let user = user.unwrap();

    let password_verification = password_hash_util::verify_password(password, &user.password_hash);
    if password_verification.is_err() || !password_verification.unwrap() {
        return Ok(None);
    }

    Ok(Some(user))
}