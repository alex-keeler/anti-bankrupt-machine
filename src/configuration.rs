use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub user_jwt_secret: String,
}