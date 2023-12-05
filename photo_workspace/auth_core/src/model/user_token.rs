
use serde::{Deserialize, Serialize};
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use std::env;
use jsonwebtoken::{DecodingKey, TokenData, Validation};

pub static THREE_DAY: i64 = 60 * 60 * 24 * 3 ;

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub uuid: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenResp {
    pub token_type: String,
    pub access_token: String,
}

impl UserToken {
    
    pub fn generate_token(uuid: &str, username: &str) -> String {

        let now = Utc::now().timestamp_nanos_opt().unwrap() / 1_000_000_000;

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let encoding_key: EncodingKey = EncodingKey::from_secret(secret.as_bytes());

        let payload = UserToken {
            iat: now,
            exp: now + THREE_DAY,
            uuid: uuid.to_string(),
            name: username.to_string(),
        };

        jsonwebtoken::encode(&Header::default(), &payload, &encoding_key).unwrap()
    }

    pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    
        jsonwebtoken::decode::<UserToken>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
    }
}




