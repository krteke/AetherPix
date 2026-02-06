use loco_rs::prelude::*;
use mime_guess2::mime;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

use crate::common::client::get_r2;
use crate::common::settings::SettingsService;
use crate::models::images;
use crate::models::tmps;
use crate::models::users::users;
use crate::views::upload::{PresignResponse, UploadResponse};
use crate::workers::thumbnail::{Worker, WorkerArgs};

const TEMP_DIR: &str = "tmp_upload";

#[derive(Debug, Deserialize)]
pub struct UploadParams {
    pub public: Option<bool>,
    pub quality: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresignParams {
    pub file_name: String,
    pub content_type: String,
    pub size: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmParams {
    pub file_name: String,
    pub size: i64,
    pub is_public: bool,
}

pub struct UploadResult {
    pub url: String,
    pub file_name: String,
    pub is_public: bool,
    pub user_id: Option<Uuid>,
    pub uuid: Uuid,
    pub raw_name: String,
    // pub size: String,
    // pub status: String,
}

async fn upload(
    State(ctx): State<AppContext>,
    Query(upload_params): Query<UploadParams>,
    multipart: Multipart,
) -> Result<Response> {
    if !SettingsService::allow_everyone_upload().await {
        return Err(Error::Unauthorized("Upload is not allowed".to_string()));
    }

    match upload_files(multipart, &ctx, upload_params.quality).await {
        Ok(r) => {
            images::Model::save_local_with_result(&ctx.db, &r).await?;
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
    Query(upload_params): Query<UploadParams>,
    multipart: Multipart,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &jwt.claims.pid).await?;
    let public = upload_params.public.unwrap_or(true);

    match upload_files(multipart, &ctx, upload_params.quality).await {
        Ok(mut r) => {
            r.is_public = public;
            r.user_id = Some(user.pid);

            images::Model::save_local_with_result(&ctx.db, &r).await?;
            let res = if public {
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
    Query(upload_params): Query<UploadParams>,
    multipart: Multipart,
) -> Result<Response> {
    format::json(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TempFileGuard(pub PathBuf);

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
    ctx: &AppContext,
    quality: u8,
) -> Result<UploadResult> {
    tokio::fs::create_dir_all(TEMP_DIR).await?;
    // let client = get_garage();

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
        let mut avif_name = String::with_capacity(41);
        let _ = write!(file_name, "{}.{}", uuid, ext);
        let _ = write!(avif_name, "{}.avif", uuid);

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

        // let size = format!(
        //     "{:.2} MB",
        //     tmp_path.metadata()?.len() as f32 / 1024.0 / 1024.0
        // );

        let mime = mime_guess2::from_path(&tmp_path).first_or_octet_stream();
        if mime.type_() != mime::IMAGE {
            return Err(Error::BadRequest("Invalid file type".to_string()));
        }

        // let body = ByteStream::from_path(&tmp_path).await.map_err(|e| {
        //     tracing::error!("Failed to read file: {}", e);
        //     Error::InternalServerError
        // })?;

        // let result = client
        //     .pub_object(
        //         &file_name,
        //         body,
        //         mime.as_ref(),
        //         crate::common::client::Position::Original,
        //     )
        //     .await;

        let quality = quality.min(100);
        let args = WorkerArgs {
            preview_key: avif_name.clone(),
            tmp_file_guard,
            quality,
        };

        if let Err(e) = Worker::perform_later(ctx, args).await {
            tracing::error!("Failed to enqueue worker task: {}", e);
        }

        // match result {
        //     Ok(_) => {
        // tracing::debug!("Ok, uploaded file to S3");
        let local_base_url = SettingsService::local_base_url().await;
        let url = if local_base_url.trim().is_empty() {
            ctx.config.server.full_url() + "/api/view"
        } else {
            local_base_url.to_string()
        };

        return Ok(UploadResult {
            url: format!("{}/{}", url, avif_name),
            file_name: avif_name,
            is_public: true,
            user_id: None,
            uuid,
            raw_name,
            // size,
        });
        // }
        // Err(e) => {
        //     tracing::error!("Failed to upload file to S3: {}", e);
        //     return Err(Error::InternalServerError);
        // }
        // }
    }

    Err(Error::BadRequest("No file".to_string()))
}

async fn presign(
    State(_ctx): State<AppContext>,
    Json(params): Json<PresignParams>,
) -> Result<Response> {
    if !SettingsService::allow_everyone_upload().await {
        return Err(Error::Unauthorized("Upload not allowed".to_string()));
    }

    match presign_file(&params.file_name, &params.content_type).await {
        Ok((_, url)) => format::json(PresignResponse { upload_url: url }),
        Err(e) => {
            tracing::error!("Failed to presign file: {}", e);
            Err(Error::InternalServerError)
        }
    }
}

async fn presign_with_jwt(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<PresignParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    match presign_file(&params.file_name, &params.content_type).await {
        Ok((file_name, url)) => {
            tmps::Model::create_tmp_record(&ctx.db, user.pid, &file_name).await?;

            format::json(PresignResponse { upload_url: url })
        }
        Err(e) => {
            tracing::error!("Failed to presign file: {}", e);
            Err(Error::InternalServerError)
        }
    }
}

async fn presign_with_token(
    auth: auth::ApiToken<users::Model>,
    State(ctx): State<AppContext>,
    Json(params): Json<PresignParams>,
) -> Result<Response> {
    todo!()
}

async fn presign_file(file_name: &str, content_type: &str) -> Result<(String, String)> {
    let client = get_r2();

    let ext = Path::new(file_name)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let key = format!("{}.{}", Uuid::new_v4(), ext);
    let presigned_req = client.presign(&key, content_type, 300).await.map_err(|e| {
        tracing::error!("Failed to presign: {}", e.to_string());
        Error::InternalServerError
    })?;

    Ok((key, presigned_req.uri().to_string()))
}

async fn confirm(
    State(_ctx): State<AppContext>,
    Json(params): Json<PresignParams>,
) -> Result<Response> {
    todo!()
}

async fn confirm_with_jwt() -> Result<Response> {
    todo!()
}

async fn confirm_with_token() -> Result<Response> {
    todo!()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/upload", post(upload))
        .add("/upload/jwt", post(upload_with_jwt))
        .add("/upload/token", post(upload_with_token))
        .add("/presign", post(presign))
        .add("/presign/jwt", post(presign_with_jwt))
        .add("/presign/token", post(presign_with_token))
        .add("/confirm", post(confirm))
        .add("/confirm/jwt", post(confirm_with_jwt))
        .add("/confirm/token", post(confirm_with_token))
}
