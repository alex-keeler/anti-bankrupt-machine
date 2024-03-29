use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::Error;
use rocket::serde::{Deserialize, Serialize};
use rocket::{State};
use crate::configuration::AppConfig;
use crate::models::users::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub id: i64
}

impl From<User> for UserClaims {
    fn from(user: User) -> Self {
        UserClaims {
            id: user.id.unwrap(),
        }
    }
}

pub fn build_user_jwt(user_claims: UserClaims, config: &State<AppConfig>) -> Result<String, Error> {

    let key = EncodingKey::from_base64_secret(config.user_jwt_secret.as_str())?;
    let header = Header::new(Algorithm::HS512);

    let token = encode(&header, &user_claims, &key)?;

    Ok(token)
}

pub fn parse_user_jwt(jwt: &str, config: &State<AppConfig>) -> Result<UserClaims, Error> {

    let key = DecodingKey::from_base64_secret(config.user_jwt_secret.as_str())?;

    let token_data = decode::<UserClaims>(&jwt, &key, &Validation::default())?;

    Ok(token_data.claims)
}