use aws_sdk_s3::primitives::ByteStream;
use axum::Extension;
use image::ImageReader;
use loco_rs::prelude::*;
use mime_guess2::mime;
use serde::Deserialize;
use std::fmt::Write;
use std::io::Cursor;
use std::path::PathBuf;
use std::{path::Path, sync::Arc};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

use crate::common::client::S3Client;
use crate::common::settings::SettingsService;
use crate::models::images;
use crate::models::users::users;
use crate::views::upload::UploadResponse;

const TEMP_DIR: &str = "tmp_upload";

#[derive(Debug, Deserialize)]
pub struct UploadParams {
    pub public: bool,
}

pub struct UploadResult {
    pub url: String,
    pub file_name: String,
    pub is_public: bool,
    pub user_id: Option<Uuid>,
    pub uuid: Uuid,
    pub raw_name: String,
    pub size: String,
    // pub status: String,
}

async fn upload(
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<Arc<S3Client>>,
    // Query(upload_params): Query<UploadParams>,
    multipart: Multipart,
) -> Result<Response> {
    if !SettingsService::allow_everyone_upload().await {
        return Err(Error::Unauthorized("Upload is not allowed".to_string()));
    }
    let base_url = ctx.config.server.full_url();

    match upload_files(multipart, s3_client.clone(), &base_url).await {
        Ok(r) => {
            images::Model::create_with_upload_result(&ctx.db, &r).await?;
            format::json(UploadResponse::from(r))
        }
        Err(e) => {
            tracing::error!("Failed to upload files: {}", e);

            Err(Error::InternalServerError)
        }
    }
}

async fn upload_with_jwt(
    jwt: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<Arc<S3Client>>,
    Query(upload_params): Query<UploadParams>,
    multipart: Multipart,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &jwt.claims.pid).await?;
    let base_url = ctx.config.server.full_url();

    match upload_files(multipart, s3_client.clone(), &base_url).await {
        Ok(mut r) => {
            r.is_public = upload_params.public;
            r.user_id = Some(user.pid);

            images::Model::create_with_upload_result(&ctx.db, &r).await?;
            let res = if upload_params.public {
                UploadResponse { url: Some(r.url) }
            } else {
                UploadResponse { url: None }
            };
            format::json(res)
        }
        Err(e) => {
            tracing::error!("Failed to upload files: {}", e);

            Err(Error::InternalServerError)
        }
    }
}

// TODO: implement upload_with_token
async fn upload_with_token(
    auth: auth::ApiToken<users::Model>,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<Arc<S3Client>>,
    Query(upload_params): Query<UploadParams>,
    multipart: Multipart,
) -> Result<Response> {
    format::json(())
}

struct TempFileGuard(PathBuf);

impl Drop for TempFileGuard {
    fn drop(&mut self) {
        let path = self.0.clone();
        if path.exists() {
            tokio::spawn(async move {
                if let Err(e) = fs::remove_file(&path).await {
                    tracing::warn!("Failed to clean up temp file {}: {}", path.display(), e);
                }
            });
        }
    }
}

async fn upload_files(
    mut multipart: Multipart,
    client: Arc<S3Client>,
    base_url: &str,
) -> Result<UploadResult> {
    tokio::fs::create_dir_all(TEMP_DIR).await?;

    if let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|e| Error::BadRequest(e.to_string()))?
    {
        let raw_name = field
            .file_name()
            .and_then(|p| Path::new(p).file_name().and_then(|e| e.to_str()))
            .unwrap_or("")
            .to_string();
        let ext = field
            .file_name()
            .and_then(|p| Path::new(p).extension().and_then(|e| e.to_str()))
            .unwrap_or("");

        let uuid = Uuid::now_v7();
        let mut file_name = String::with_capacity(37 + ext.len());
        let mut preview_name = String::with_capacity(41);
        let _ = write!(file_name, "{}.{}", uuid, ext);
        let _ = write!(preview_name, "{}.webp", uuid);

        let tmp_path = Path::new(TEMP_DIR).join(&file_name);
        let mut tmp_file = File::create(&tmp_path).await?;

        let tmp_file_guard = TempFileGuard(tmp_path.clone());

        while let Some(chunk) = field
            .chunk()
            .await
            .map_err(|e| Error::BadRequest(e.to_string()))?
        {
            tmp_file.write_all(&chunk).await?;
        }
        tmp_file.flush().await?;

        let size = format!(
            "{:.2} MB",
            tmp_path.metadata()?.len() as f32 / 1024.0 / 1024.0
        );

        let mime = mime_guess2::from_path(&tmp_path).first_or_octet_stream();
        if mime.type_() != mime::IMAGE {
            return Err(Error::BadRequest("Invalid file type".to_string()));
        }

        let upload_original_task = async {
            let body = ByteStream::from_path(&tmp_path)
                .await
                .map_err(|e| e.to_string())?;

            client
                .pub_object(
                    &file_name,
                    body,
                    mime.as_ref(),
                    crate::common::client::Position::Original,
                )
                .await
                .map_err(|e| e.to_string())
        };

        let tmp_path_clone = tmp_path.clone();
        let client = client.clone();
        let upload_preview_task = async move {
            let thumbnail_data =
                tokio::task::spawn_blocking(move || process_thumbnail(tmp_path_clone))
                    .await
                    .map_err(|e| e.to_string())?
                    .map_err(|e| e.to_string())?;
            let body = ByteStream::from(thumbnail_data);

            client
                .pub_object(
                    &preview_name,
                    body,
                    "image/webp",
                    crate::common::client::Position::Preview,
                )
                .await
                .map_err(|e| e.to_string())
        };

        let result = tokio::try_join!(upload_original_task, upload_preview_task);

        drop(tmp_file_guard);

        match result {
            Ok(_) => {
                tracing::debug!("Ok, uploaded file to S3");
                return Ok(UploadResult {
                    url: format!("{}/api/view/{}", base_url, file_name),
                    file_name,
                    is_public: true,
                    user_id: None,
                    uuid,
                    raw_name,
                    size,
                });
            }
            Err(e) => {
                tracing::error!("Failed to upload file to S3: {}", e);
                return Err(Error::InternalServerError);
            }
        }
    }

    Err(Error::BadRequest("No file".to_string()))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/upload", post(upload))
        .add("/upload/jwt", post(upload_with_jwt))
        .add("/upload/token", post(upload_with_token))
}

fn process_thumbnail(file_path: PathBuf) -> Result<Vec<u8>, String> {
    let img = ImageReader::open(file_path)
        .map_err(|e| e.to_string())?
        .with_guessed_format()
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    let thumbnail = img.resize(400, 400, image::imageops::FilterType::Triangle);

    let mut buffer = Cursor::new(Vec::new());
    thumbnail
        .write_to(&mut buffer, image::ImageFormat::WebP)
        .map_err(|e| e.to_string())?;

    Ok(buffer.into_inner())
}
