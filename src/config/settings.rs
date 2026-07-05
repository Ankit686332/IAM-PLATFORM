use std::env;

pub struct Settings {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
}

impl Settings {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").unwrap(),
            host: env::var("HOST").unwrap(),
            port: env::var("PORT").unwrap().parse().unwrap(),
            jwt_secret: env::var("JWT_SECRET").unwrap(),
        }
    }
}