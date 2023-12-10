use rocket::http::Status;
use rocket::Request;
use serde::{Serialize, Deserialize};
use rocket::request::{FromRequest, Outcome};
use rocket_db_pools::Connection;
use sqlx::FromRow;
use crate::services::user_service;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub password_hash: String,
    pub is_active: bool,
    pub email_address: String,
    pub first_name: String,
    pub last_name: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = UserError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        let user_id = request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok());

        let result = match user_id {
            Some(user_id) => {
                match Connection::from_request(request).await {
                    Outcome::Success(conn) => {
                        match user_service::find_user_by_id(user_id, conn).await {
                            Ok(user) => {
                                match user.is_active {
                                    true => Outcome::Success(user),
                                    false => Outcome::Failure((Status::Unauthorized, UserError::InactiveUser)),
                                }
                            },
                            Err(_) => Outcome::Failure((Status::Unauthorized, UserError::InvalidAuthToken)),
                        }
                    },
                    _ => Outcome::Failure((Status::ServiceUnavailable, UserError::ConnectionError)),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, UserError::MissingAuthToken)),
        };

        result
    }
}

#[derive(Debug)]
pub enum UserError {
    MissingAuthToken,
    InvalidAuthToken,
    InactiveUser,
    ConnectionError,
}