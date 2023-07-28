use serde::Deserialize;

use crate::entity::user::User;

#[derive(Deserialize)]
pub struct UserLoginDto {
    pub username: String,
    pub password: String,
}
