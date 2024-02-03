use rocket::http::Status;
use rocket::{Request, response};
use rocket::response::Responder;
use sqlx::Error;
use crate::models::data::application_error::ApplicationError::{RepositoryError, ServiceError};

#[derive(Debug)]
pub enum ApplicationError {
    ServiceError(String),
    RepositoryError(Error),
}

impl From<Error> for ApplicationError {
    fn from(value: Error) -> Self {
        RepositoryError(value)
    }
}

impl From<String> for ApplicationError {
    fn from(value: String) -> Self {
        ServiceError(value)
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