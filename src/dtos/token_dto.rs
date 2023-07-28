use serde::Serialize;

#[derive(Serialize)]
pub struct TokenDto {
    pub token: String,
}

impl TokenDto {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
