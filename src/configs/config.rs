use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub app_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            app_port: env::var("NODE_PORT")
                .expect("DATABASE_URL must be set")
                .parse()
                .unwrap(),
        }
    }
}
