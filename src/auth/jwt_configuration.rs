#[derive(Debug, Clone)]
pub struct JwtConfiguration {
    pub secret: String,
    pub expiration: u32,
}
