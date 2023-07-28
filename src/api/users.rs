use actix_web::{
    post,
    web::{self, Json},
    HttpResponse,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{
    auth::token_claims::TokenClaims,
    dtos::{token_dto::TokenDto, user_login_dto::UserLoginDto, user_register_dto::UserRegisterDto},
    entity::user::User,
    repository::user_repository,
    AppState,
};

use super::errors::ApiError;

type Result<T> = std::result::Result<T, ApiError>;

#[post("/register")]
pub async fn register(
    data: web::Data<AppState>,
    body: Json<UserRegisterDto>,
) -> Result<HttpResponse> {
    let user: User = body.into_inner().into();

    let result = user_repository::create_async(&data.db, user).await?;

    return Ok(HttpResponse::Ok().json(result));
}

#[post("/login")]
pub async fn login(data: web::Data<AppState>, body: Json<UserLoginDto>) -> Result<HttpResponse> {
    let login_dto = body.into_inner();

    let result = user_repository::get_by_username_async(&data.db, login_dto.username)
        .await
        .map_err(|_| ApiError::UnauthorizedError)?;

    let hash = PasswordHash::new(&result.password).unwrap();

    let verification = Argon2::default().verify_password(login_dto.password.as_bytes(), &hash);

    if verification.is_err() {
        return Err(ApiError::UnauthorizedError);
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::seconds(data.jwt.expiration as i64)).timestamp() as usize;
    let claims = TokenClaims {
        sub: result.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.jwt.secret.as_ref()),
    )
    .unwrap();

    return Ok(HttpResponse::Ok().json(TokenDto::new(token)));
}

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("users")
        .service(register)
        .service(login);

    cfg.service(scope);
}
