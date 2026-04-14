use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn init_db() -> PgPool {
    let database_url = "postgres://dev:123456@localhost/rchat";
    PgPoolOptions::new()
        .max_connections(80)
        .min_connections(5)
        .connect(database_url)
        .await
        .expect("database connect fails")
}
