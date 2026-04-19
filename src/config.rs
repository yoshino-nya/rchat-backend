use std::env::var;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub base_url: String,
    pub db_url: String,
    pub port: u16,
}

impl AppConfig {
    pub fn load_from_env() -> Self {
        Self {
            base_url: var("BASE_URL").expect("BASE_URL not set"),
            db_url: var("DB_URL").expect("DB_URL not set"),
            port: var("PORT").unwrap().parse().expect("PORT not set"),
        }
    }
    pub fn bind_addr(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
