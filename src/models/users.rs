use std::error::Error;
use jsonwebtoken::errors::{ErrorKind};
use rocket::http::Status;
use rocket::{Request, State};
use serde::{Serialize, Deserialize};
use rocket::request::{FromRequest, Outcome};
use rocket_db_pools::Connection;
use sqlx::FromRow;
use crate::configuration::AppConfig;
use crate::services::user_service;
use crate::services::user_token_service;

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

        let user_jwt = request.headers()
            .get_one("Auth-Token");

        if user_jwt.is_none() {
            return Outcome::Failure((Status::Unauthorized, UserError::MissingAuthToken));
        }
        let user_jwt = user_jwt.unwrap();

        let config = request.guard::<&State<AppConfig>>().await;
        if config.is_failure() {
            println!("Config fetch failed.");
            return Outcome::Failure((Status::InternalServerError, UserError::InternalValidationError));
        }
        let config = config.unwrap();

        let user_claims = user_token_service::parse_user_jwt(user_jwt, config);

        match user_claims {
            Ok(user_claims) => {
                match Connection::from_request(request).await {
                    Outcome::Success(conn) => {
                        match user_service::find_user_by_id(user_claims.id, conn).await {
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
            Err(err) => {
                println!("Could not get authorized user: {:?} ({:?})", err.kind(), err.source());

                match err.kind() {
                    ErrorKind::InvalidToken |
                    ErrorKind::InvalidAlgorithmName |
                    ErrorKind::MissingRequiredClaim(_) |
                    ErrorKind::ExpiredSignature |
                    ErrorKind::InvalidIssuer |
                    ErrorKind::InvalidAudience |
                    ErrorKind::InvalidSubject |
                    ErrorKind::ImmatureSignature |
                    ErrorKind::InvalidAlgorithm => Outcome::Failure((Status::Unauthorized, UserError::InvalidAuthToken)),
                    _ => Outcome::Failure((Status::InternalServerError, UserError::InternalValidationError)),
                }
            }
        }
    }
}

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for User {
//     type Error = UserError;
//
//     async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//
//         let user_id = request.cookies()
//             .get_private("user_id")
//             .and_then(|cookie| cookie.value().parse().ok());
//
//         let result = match user_id {
//             Some(user_id) => {
//                 match Connection::from_request(request).await {
//                     Outcome::Success(conn) => {
//                         match user_service::find_user_by_id(user_id, conn).await {
//                             Ok(user) => {
//                                 match user.is_active {
//                                     true => Outcome::Success(user),
//                                     false => Outcome::Failure((Status::Unauthorized, UserError::InactiveUser)),
//                                 }
//                             },
//                             Err(_) => Outcome::Failure((Status::Unauthorized, UserError::InvalidAuthToken)),
//                         }
//                     },
//                     _ => Outcome::Failure((Status::ServiceUnavailable, UserError::ConnectionError)),
//                 }
//             }
//             None => Outcome::Failure((Status::Unauthorized, UserError::MissingAuthToken)),
//         };
//
//         result
//     }
// }

#[derive(Debug)]
pub enum UserError {
    InternalValidationError,
    MissingAuthToken,
    InvalidAuthToken,
    InactiveUser,
    ConnectionError,
}