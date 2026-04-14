use std::path::PathBuf;

use axum::extract::Multipart;
use image::load_from_memory;
use uuid::Uuid;

use crate::models::upload::UploadError;

pub async fn upload_avatar(mut multipart: Multipart) -> Result<(), UploadError> {
    let field = multipart.next_field().await?.ok_or(UploadError::Nofile)?;
    // let content_type = field
    //     .content_type()
    //     .ok_or(UploadError::MissingContentType)?
    //     .to_string();
    // if !content_type.starts_with("image/") {
    //     return Err(UploadError::InvalidFileType);
    // }
    let data = field.bytes().await?;

    let upload_dir = PathBuf::from("./uploads/avatars");
    // tokio::fs::create_dir_all(&upload_dir).await?;

    let img = load_from_memory(&data).map_err(|_| UploadError::InvalidFileType)?;
    let rgb = img.to_rgb8();

    let filename = format!("{}.jpg", Uuid::new_v4());
    let filepath = upload_dir.join(filename);

    tracing::info!(?filepath);

    let file = tokio::fs::File::create(&filepath).await?;
    let std_file = file.into_std().await;
    let mut writer = std::io::BufWriter::new(std_file);

    rgb.write_to(&mut writer, image::ImageFormat::Jpeg)
        .map_err(|_| UploadError::InvalidFileType)?;
    Ok(())
}
