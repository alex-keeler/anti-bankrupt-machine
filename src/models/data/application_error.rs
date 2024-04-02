use rocket::http::Status;
use rocket::{Request, response};
use rocket::response::Responder;
use crate::models::data::application_error::ApplicationError::{JwtAuthenticationError, RepositoryError, ServiceError};

#[derive(Debug)]
pub enum ApplicationError {
    RepositoryError(sqlx::Error),
    ServiceError(String),
    JwtAuthenticationError(jsonwebtoken::errors::Error),
}

impl From<sqlx::Error> for ApplicationError {
    fn from(value: sqlx::Error) -> Self {
        RepositoryError(value)
    }
}

impl From<String> for ApplicationError {
    fn from(value: String) -> Self {
        ServiceError(value)
    }
}

impl From<jsonwebtoken::errors::Error> for ApplicationError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        JwtAuthenticationError(value)
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApplicationError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        // log `self` to your favored error tracker, e.g.
        // sentry::capture_error(&self);

        match self {
            _ => Status::InternalServerError.respond_to(req)
        }
    }
}