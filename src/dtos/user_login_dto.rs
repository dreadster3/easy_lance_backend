use serde::Deserialize;



#[derive(Deserialize)]
pub struct UserLoginDto {
    pub username: String,
    pub password: String,
}
