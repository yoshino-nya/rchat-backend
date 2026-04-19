use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::config::AppConfig;

pub async fn init_db(config: &AppConfig) -> PgPool {
    PgPoolOptions::new()
        .max_connections(80)
        .min_connections(5)
        .connect(&config.db_url)
        .await
        .expect("database connect fails")
}
