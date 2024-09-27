use rocket::http::Status;
use rocket::{Request, response};
use rocket::response::Responder;
use crate::models::data::application_error::ApplicationError::{CsvParseError, IntParseError, JwtAuthenticationError, RepositoryError, ServiceError};

#[derive(Debug)]
pub enum ApplicationError {
    RepositoryError(sqlx::Error),
    CsvParseError(csv::Error),
    IntParseError(std::num::ParseIntError),
    ServiceError(String),
    AuthenticationError(String),
    JwtAuthenticationError(jsonwebtoken::errors::Error),
}

impl From<sqlx::Error> for ApplicationError {
    fn from(value: sqlx::Error) -> Self {
        RepositoryError(value)
    }
}

impl From<csv::Error> for ApplicationError {
    fn from(value: csv::Error) -> Self {
        CsvParseError(value)
    }
}

impl From<std::num::ParseIntError> for ApplicationError {
    fn from(value: std::num::ParseIntError) -> Self {
        IntParseError(value)
    }
}

impl From<String> for ApplicationError {
    fn from(value: String) -> Self {
        ServiceError(value)
    }
}

impl From<&str> for ApplicationError {
    fn from(value: &str) -> Self {
        ServiceError(String::from(value))
    }
}

impl From<jsonwebtoken::errors::Error> for ApplicationError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        JwtAuthenticationError(value)
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApplicationError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        match self {
            ApplicationError::AuthenticationError(_) => Status::Unauthorized.respond_to(req),
            _ => Status::InternalServerError.respond_to(req)
        }
    }
}