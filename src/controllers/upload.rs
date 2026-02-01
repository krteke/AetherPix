use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use axum::Extension;
use loco_rs::prelude::*;
use std::fmt::Write;
use std::{path::Path, sync::Arc};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

use crate::common::settings::SettingsService;
use crate::models::users::users;

const TEMP_DIR: &str = "tmp_upload";

pub struct S3Client {
    client: Client,
    bucket: String,
}

impl S3Client {
    pub fn new(client: Client, bucket: String) -> Self {
        Self { client, bucket }
    }
}

async fn upload(
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<Arc<S3Client>>,
    multipart: Multipart,
) -> Result<Response> {
    if !SettingsService::allow_everyone_upload().await {
        return Err(Error::Unauthorized("Upload is not allowed".to_string()));
    }

    match upload_files(multipart, s3_client.clone(), &s3_client.bucket).await {
        Ok(r) => {}
        Err(e) => {
            tracing::error!("Failed to upload files: {}", e);
        }
    }

    format::json(())
}

async fn upload_with_jwt(
    jwt: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<Arc<S3Client>>,
    multipart: Multipart,
) -> Result<Response> {
    todo!()
}

async fn upload_with_token(
    auth: auth::ApiToken<users::Model>,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<Arc<S3Client>>,
    multipart: Multipart,
) -> Result<Response> {
    todo!()
}

async fn upload_files(mut multipart: Multipart, client: Arc<S3Client>, bucket: &str) -> Result<()> {
    tokio::fs::create_dir_all(TEMP_DIR).await?;

    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|e| Error::BadRequest(e.to_string()))?
    {
        let ext = field
            .file_name()
            .and_then(|p| Path::new(p).extension().and_then(|e| e.to_str()))
            .unwrap_or("");

        let file_name = gen_filename(ext);
        let tmp_path = Path::new(TEMP_DIR).join(&file_name);
        let mut tmp_file = File::create(&tmp_path).await?;

        while let Some(chunk) = field
            .chunk()
            .await
            .map_err(|e| Error::BadRequest(e.to_string()))?
        {
            tmp_file.write_all(&chunk).await?;
        }
        tmp_file.flush().await?;

        let body = ByteStream::from_path(&tmp_path)
            .await
            .map_err(|_| Error::InternalServerError)?;
        let mime = mime_guess2::from_path(&tmp_path).first_or_octet_stream();

        let s3_result = client
            .client
            .put_object()
            .bucket(bucket)
            .key(file_name)
            .body(body)
            .content_type(mime.as_ref())
            .send()
            .await;

        let _ = fs::remove_file(tmp_path).await;

        match s3_result {
            Ok(_) => {
                // TODO: Implement success response
                tracing::debug!("Ok, uploaded file to S3");
                // return Ok(());
            }
            Err(e) => {
                tracing::error!("Failed to upload file to S3: {}", e);
                // TODO: Implement error response
            }
        }
    }

    Err(Error::BadRequest("No file".to_string()))
}

/// generate filename
fn gen_filename(ext: &str) -> String {
    let uuid = Uuid::now_v7();
    let mut s = String::with_capacity(37 + ext.len());
    let _ = write!(s, "{}.{}", uuid, ext);

    s
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/upload", post(upload))
        .add("/upload/jwt", post(upload_with_jwt))
        .add("/upload/token", post(upload_with_token))
}
