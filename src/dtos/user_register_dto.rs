use serde::Deserialize;

use crate::entity::user::User;

#[derive(Deserialize)]
pub struct UserRegisterDto {
    username: String,
    email: String,
    password: String,
}

impl Into<User> for UserRegisterDto {
    fn into(self) -> User {
        User::new(self.username, self.email, self.password)
    }
}
