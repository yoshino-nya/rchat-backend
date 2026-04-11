use sqlx::PgPool;

pub async fn init_db() -> PgPool {
    let database_url = "postgres://dev:123456@localhost/rchat";
    PgPool::connect(database_url)
        .await
        .expect("database connect fails!")
}
