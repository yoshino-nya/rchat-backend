use sqlx::PgPool;

use crate::models::relationship::RelationshipResponse;

pub async fn get_relationship_service(
    pool: &PgPool,
    user_low: i32,
    user_high: i32,
) -> Result<RelationshipResponse, sqlx::Error> {
    let res: bool = sqlx::query_scalar(
        r#"
    SELECT EXISTS(SELECT 1 FROM friendship WHERE user_low = $1 AND user_high = $2)"#,
    )
    .bind(user_low)
    .bind(user_high)
    .fetch_one(pool)
    .await?;

    Ok(RelationshipResponse { is_friend: res })
}
