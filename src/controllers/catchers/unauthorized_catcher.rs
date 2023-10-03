use rocket::{Either, Request};
use rocket::request::Outcome;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

use crate::models::users::{User, UserError};

#[catch(401)]
pub async fn redirect_or_access_denied(req: &Request<'_>) -> Either<Template, Redirect> {
    let user_status: Outcome<User, UserError> = req.guard().await;
    if user_status.failed().is_some_and(|status| matches!(status.1, UserError::MissingAuthToken)) {
        return Either::Right(Redirect::to(uri!(crate::controllers::login_controller::get_login_form)));
    }
    Either::Left(Template::render("catchers/access-denied", context! {}))
}