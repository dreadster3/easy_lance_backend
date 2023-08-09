use chrono;
use thiserror::Error;

use super::{jwt_configuration::JwtConfiguration, token_claims::TokenClaims};

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("Token is invalid")]
    DecodeTokenError,
}

type Result<T> = std::result::Result<T, TokenError>;

pub struct Identity {
    user_id: i32,
}

impl Identity {
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    pub fn get_claims(&self, validity_in_secs: i64) -> TokenClaims {
        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + chrono::Duration::seconds(validity_in_secs)).timestamp() as usize;

        TokenClaims {
            sub: self.user_id.to_string(),
            exp,
            iat,
        }
    }

    pub fn generate_token(&self, configuration: &JwtConfiguration) -> String {
        let claims = self.get_claims(configuration.expiration as i64);

        return jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(configuration.secret.as_ref()),
        )
        .unwrap();
    }

    pub fn decode_token(token: &str, secret: &str) -> Result<Identity> {
        let token_details = match jsonwebtoken::decode::<TokenClaims>(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        ) {
            Ok(token) => token,
            Err(_) => return Err(TokenError::DecodeTokenError),
        };

        Ok(Identity::new(
            token_details.claims.sub.parse::<i32>().unwrap(),
        ))
    }
}
