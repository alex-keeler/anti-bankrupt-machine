use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation, decode_header};
use jsonwebtoken::errors::Error;
use rocket::serde::{Deserialize, Serialize};
use rocket::{State};
use crate::configuration::AppConfig;
use crate::models::users::User;

const EXPIRATION_SECONDS: u64 = 60 * 60 * 2;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub id: i64,
    pub iat: u64,
    pub exp: u64,
}

impl From<User> for UserClaims {
    fn from(user: User) -> Self {
        let iat = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        UserClaims {
            id: user.id.unwrap(),
            iat,
            exp: iat + EXPIRATION_SECONDS,
        }
    }
}

pub fn build_user_jwt(user_claims: UserClaims, config: &State<AppConfig>) -> Result<String, Error> {

    // TODO: initialize key once for better performance
    let key = EncodingKey::from_base64_secret(config.user_jwt_secret.as_str())?;
    let header = Header::new(Algorithm::HS256);

    let token = encode(&header, &user_claims, &key)?;

    Ok(token)
}

pub fn parse_user_jwt(jwt: &str, config: &State<AppConfig>) -> Result<UserClaims, Error> {

    // TODO: initialize key once for better performance
    let key = DecodingKey::from_base64_secret(config.user_jwt_secret.as_str())?;

    let token_data = decode::<UserClaims>(&jwt, &key, &Validation::default())?;

    Ok(token_data.claims)
}