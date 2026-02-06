use std::{
    error::Error,
    sync::{Arc, OnceLock},
    time::Duration,
};

use aws_config::{Region, SdkConfig};
use aws_sdk_s3::{
    Client,
    config::{Credentials, SharedCredentialsProvider, http::HttpResponse},
    error::SdkError,
    operation::{
        get_object::{GetObjectError, GetObjectOutput},
        put_object::{PutObjectError, PutObjectOutput},
    },
    presigning::{PresignedRequest, PresigningConfig},
    primitives::ByteStream,
};
use loco_rs::app::AppContext;

use crate::common::settings::SettingsService;

static S3_GARAGE: OnceLock<Arc<S3Client>> = OnceLock::new();
static S3_R2: OnceLock<Arc<R2Client>> = OnceLock::new();

pub async fn init_garage(ctx: &AppContext) {
    if S3_GARAGE.get().is_some() {
        return;
    }

    SettingsService::load(&ctx.db)
        .await
        .expect("加载系统配置失败");

    let origin_bucket_name = SettingsService::origin_bucket_name().await;
    let preview_bucket_name = SettingsService::preview_bucket_name().await;
    let avif_bucket_name = SettingsService::avif_bucket_name().await;
    let s3_client =
        Arc::new(S3Client::new(origin_bucket_name, preview_bucket_name, avif_bucket_name).await);

    S3_GARAGE.set(s3_client).expect("初始化S3客户端失败");
}

pub async fn init_r2(ctx: &AppContext) {
    if S3_R2.get().is_some() {
        return;
    }

    SettingsService::load(&ctx.db)
        .await
        .expect("加载系统配置失败");

    let bucket_name = SettingsService::r2_bucket_name().await;
    let r2_client = Arc::new(R2Client::new(bucket_name).await);

    S3_R2.set(r2_client).expect("初始化R2客户端失败");
}

pub async fn reload() -> Result<(), String> {
    let origin_bucket_name = SettingsService::origin_bucket_name().await;
    let preview_bucket_name = SettingsService::preview_bucket_name().await;
    let avif_bucket_name = SettingsService::avif_bucket_name().await;
    let s3_client =
        Arc::new(S3Client::new(origin_bucket_name, preview_bucket_name, avif_bucket_name).await);

    S3_GARAGE
        .set(s3_client)
        .map_err(|_| "Reload Garage failed")?;

    let bucket_name = SettingsService::r2_bucket_name().await;
    let r2_client = Arc::new(R2Client::new(bucket_name).await);

    S3_R2.set(r2_client).map_err(|_| "Reload R2 failed")?;

    Ok(())
}

pub fn get_garage() -> &'static Arc<S3Client> {
    S3_GARAGE.get().expect("S3客户端未初始化")
}

pub fn get_r2() -> &'static Arc<R2Client> {
    S3_R2.get().expect("R2客户端未初始化")
}

#[derive(Debug)]
pub struct S3Client {
    client: Client,
    origin_bucket: String,
    preview_bucket: String,
    avif_bucket: String,
}

pub enum Position {
    Original,
    Preview,
    Avif,
}

impl S3Client {
    pub async fn new(origin_bucket: String, preview_bucket: String, avif_bucket: String) -> Self {
        let client = init_s3_client().await;
        Self {
            client,
            origin_bucket,
            preview_bucket,
            avif_bucket,
        }
    }

    fn position(&self, position: Position) -> &str {
        match position {
            Position::Original => &self.origin_bucket,
            Position::Preview => &self.preview_bucket,
            Position::Avif => &self.avif_bucket,
        }
    }

    pub async fn pub_object(
        &self,
        key: &str,
        body: ByteStream,
        content_type: &str,
        position: Position,
    ) -> Result<PutObjectOutput, SdkError<PutObjectError, HttpResponse>> {
        self.client
            .put_object()
            .bucket(self.position(position))
            .key(key)
            .body(body)
            .content_type(content_type)
            .send()
            .await
    }

    pub async fn get_object(
        &self,
        key: &str,
        position: Position,
    ) -> Result<GetObjectOutput, SdkError<GetObjectError, HttpResponse>> {
        self.client
            .get_object()
            .bucket(self.position(position))
            .key(key)
            .send()
            .await
    }
}

async fn init_s3_client() -> Client {
    let region = SettingsService::aws_region().await;
    let endpoint_url = SettingsService::aws_endpoint_url().await;
    let access_key_id = SettingsService::aws_access_key_id().await;
    let secret_access_key = SettingsService::aws_secret_access_key().await;

    build_client(region, endpoint_url, access_key_id, secret_access_key)
}

async fn init_r2_client() -> Client {
    let region = SettingsService::r2_region().await;
    let endpoint_url = SettingsService::r2_endpoint_url().await;
    let access_key_id = SettingsService::r2_access_key_id().await;
    let secret_access_key = SettingsService::r2_secret_access_key().await;

    build_client(region, endpoint_url, access_key_id, secret_access_key)
}

fn build_client(
    region: String,
    endpoint_url: String,
    access_key_id: String,
    secret_access_key: String,
) -> Client {
    let credentials = Credentials::builder()
        .access_key_id(access_key_id)
        .secret_access_key(secret_access_key)
        .provider_name("static")
        .build();

    let config = SdkConfig::builder()
        .region(Region::new(region))
        .endpoint_url(endpoint_url)
        .credentials_provider(SharedCredentialsProvider::new(credentials))
        .build();

    aws_sdk_s3::Client::new(&config)
}

#[derive(Debug)]
pub struct R2Client {
    client: Client,
    bucket: String,
}

impl R2Client {
    pub async fn new(bucket: String) -> Self {
        let client = init_r2_client().await;
        Self { client, bucket }
    }

    pub async fn presign(
        &self,
        key: &str,
        content_type: &str,
        expires_in: u64,
    ) -> Result<PresignedRequest, Box<dyn Error>> {
        let result = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .presigned(PresigningConfig::expires_in(Duration::from_secs(
                expires_in,
            ))?)
            .await?;

        Ok(result)
    }

    pub async fn sign_download_url(&self, key: &str, expires_in: u64) -> Result<String, String> {
        let presigned_req = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(
                PresigningConfig::expires_in(Duration::from_secs(expires_in))
                    .map_err(|e| e.to_string())?,
            )
            .await
            .map_err(|e| e.to_string())?;

        Ok(presigned_req.uri().to_string())
    }
}
