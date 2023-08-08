use actix_web::{
    cookie::Cookie,
    get, post,
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};


use crate::{
    auth::{token::Identity},
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

    let identity = Identity::new(result.id);

    let access_token = identity.generate_token(&data.jwt);
    let refresh_token = identity.generate_token(&data.refresh_jwt);

    // Save refresh token in database
    user_repository::set_refresh_token_async(&data.db, result.id, refresh_token.clone()).await?;

    let refresh_cookie = Cookie::build("refresh_token", refresh_token.clone())
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();

    return Ok(HttpResponse::Ok()
        .cookie(refresh_cookie)
        .json(TokenDto::new(access_token, refresh_token)));
}

#[get("/refresh")]
pub async fn refresh(data: web::Data<AppState>, request: HttpRequest) -> Result<HttpResponse> {
    let refresh_token = match request.cookie("refresh_token") {
        Some(cookie) => cookie.value().to_string(),
        None => return Err(ApiError::UnauthorizedError),
    };

    let token_identity = match Identity::decode_token(&refresh_token, &data.refresh_jwt.secret) {
        Ok(identity) => identity,
        Err(_) => return Err(ApiError::UnauthorizedError),
    };

    let user_id = token_identity.get_user_id();

    let result = user_repository::get_by_id_async(&data.db, user_id)
        .await
        .map_err(|_| ApiError::UnauthorizedError)?;

    // Get token from database
    let refresh_token_from_db = match result.refresh_token {
        Some(token) => token,
        None => return Err(ApiError::UnauthorizedError),
    };

    let hash = PasswordHash::new(&refresh_token_from_db).unwrap();

    let verification = Argon2::default().verify_password(refresh_token.as_ref(), &hash);

    // Verify token
    if verification.is_err() {
        return Err(ApiError::UnauthorizedError);
    }

    let access_token = token_identity.generate_token(&data.jwt);
    let refresh_token = token_identity.generate_token(&data.refresh_jwt);

    // Save refresh token in database
    user_repository::set_refresh_token_async(&data.db, user_id, refresh_token.clone()).await?;

    let refresh_cookie = Cookie::build("refresh_token", refresh_token.clone())
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();

    return Ok(HttpResponse::Ok()
        .cookie(refresh_cookie)
        .json(TokenDto::new(access_token, refresh_token)));
}

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("users")
        .service(register)
        .service(login)
        .service(refresh);

    cfg.service(scope);
}
