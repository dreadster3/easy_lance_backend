use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    PasswordHasher,
};

use crate::entity::user::User;

use super::errors::{NotFoundError, RepositoryError};

type Result<T> = std::result::Result<T, RepositoryError>;

pub async fn create_async(pool: &sqlx::Pool<sqlx::Postgres>, user: User) -> Result<User> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = argon2::Argon2::default()
        .hash_password(user.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let result = match sqlx::query_as!(
        User,
        r#"
        INSERT INTO tb_users (username, email, password)
        VALUES ($1, $2, $3) RETURNING *
        "#,
        user.username,
        user.email,
        hashed_password
    )
    .fetch_one(pool)
    .await
    {
        Ok(result) => Ok(result),
        Err(e) => Err(RepositoryError::InternalError(e)),
    };

    return result;
}

pub async fn get_by_username_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    username: String,
) -> Result<User> {
    let result = match sqlx::query_as!(
        User,
        r#"
        SELECT * FROM tb_users WHERE username = $1
        "#,
        username
    )
    .fetch_one(pool)
    .await
    {
        Ok(result) => Ok(result),
        Err(sqlx::Error::RowNotFound) => Err(RepositoryError::from(NotFoundError::ByProperty(
            "username".to_string(),
            username,
        ))),
        Err(e) => Err(RepositoryError::InternalError(e)),
    };

    return result;
}

pub async fn get_by_id_async(pool: &sqlx::Pool<sqlx::Postgres>, id: i32) -> Result<User> {
    let result = match sqlx::query_as!(
        User,
        r#"
        SELECT * FROM tb_users WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(result) => Ok(result),
        Err(sqlx::Error::RowNotFound) => Err(RepositoryError::from(NotFoundError::ById(id))),
        Err(e) => Err(RepositoryError::InternalError(e)),
    };

    return result;
}

pub async fn update_refresh_token_async(
    pool: &sqlx::Pool<sqlx::Postgres>,
    id: i32,
    refresh_token: String,
) -> Result<User> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_token = argon2::Argon2::default()
        .hash_password(refresh_token.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let result = match sqlx::query_as!(
        User,
        r#"
        UPDATE tb_users SET refresh_token = $1 WHERE id = $2 RETURNING *
        "#,
        hashed_token,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(result) => Ok(result),
        Err(sqlx::Error::RowNotFound) => Err(RepositoryError::from(NotFoundError::ById(id))),
        Err(e) => Err(RepositoryError::InternalError(e)),
    };

    return result;
}
