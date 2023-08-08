use serde::Serialize;

#[derive(Serialize)]
pub struct TokenDto {
    pub access_token: String,
    pub refresh_token: String,
}

impl TokenDto {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
}
