use std::path::PathBuf;

use axum::extract::Multipart;
use image::load_from_memory;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::upload::UploadError;

fn avatar_path(uuid: Uuid) -> PathBuf {
    let s = uuid.to_string();
    PathBuf::from(format!("uploads/avatars/{}/{}.jpg", &s[0..2], s))
}

pub async fn upload_avatar(
    pool: &PgPool,
    mut multipart: Multipart,
    user_id: i32,
) -> Result<Uuid, UploadError> {
    let field = multipart.next_field().await?.ok_or(UploadError::Nofile)?;

    let data = field.bytes().await?;

    let img = load_from_memory(&data).map_err(|_| UploadError::InvalidFileType)?;
    let rgb = img.to_rgb8();

    let uuid = Uuid::new_v4();

    let filepath = avatar_path(uuid);
    if let Some(parent) = filepath.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    tracing::info!(?filepath);

    let file = tokio::fs::File::create(&filepath).await?;
    let std_file = file.into_std().await;
    let mut writer = std::io::BufWriter::new(std_file);

    rgb.write_to(&mut writer, image::ImageFormat::Jpeg)
        .map_err(|_| UploadError::InvalidFileType)?;

    update_and_delete_avatar(pool, user_id, uuid).await?;
    Ok(uuid)
}

pub async fn update_and_delete_avatar(
    pool: &PgPool,
    user_id: i32,
    new_avatar: Uuid,
) -> Result<(), sqlx::Error> {
    let old_avatar: Option<Uuid> = sqlx::query_scalar(
        r#"
        WITH old AS (
            SELECT avatar FROM "user" WHERE id = $2
        )
        UPDATE "user"
        SET avatar = $1
        WHERE id = $2
        RETURNING (SELECT avatar FROM old)
    "#,
    )
    .bind(new_avatar)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    if let Some(old_avatar) = old_avatar {
        let path = avatar_path(old_avatar);
        tokio::spawn(async move {
            let res = tokio::fs::remove_file(&path).await;
            if let Err(e) = res {
                tracing::error!(%e, ?path, "删除 avatar 失败");
            }
        });
    }

    Ok(())
}
