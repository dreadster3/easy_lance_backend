use std::future;

use actix_web::{http::header, web, FromRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::AppState;

use super::token_claims::TokenClaims;

pub struct UserIdentity {
    pub id: i32,
}

impl FromRequest for UserIdentity {
    type Error = actix_web::Error;
    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        request: &actix_web::HttpRequest,
        _: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let authorization_header = match request.headers().get(header::AUTHORIZATION) {
            Some(header) => header,
            None => {
                return future::ready(Err(actix_web::error::ErrorUnauthorized(
                    "Missing authorization header",
                )))
            }
        };

        let token = match authorization_header.to_str() {
            Ok(token) => token.split_at(6).1.trim().to_string(),
            Err(_) => {
                return future::ready(Err(actix_web::error::ErrorUnauthorized(
                    "Invalid authorization header",
                )))
            }
        };

        let data = request.app_data::<web::Data<AppState>>().unwrap();

        let claims = match decode::<TokenClaims>(
            &token,
            &DecodingKey::from_secret(data.jwt.secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(claims) => claims,
            Err(_) => {
                return future::ready(Err(actix_web::error::ErrorUnauthorized("Invalid token")))
            }
        };

        return future::ready(Ok(UserIdentity {
            id: claims.claims.sub.parse::<i32>().unwrap(),
        }));
    }
}
