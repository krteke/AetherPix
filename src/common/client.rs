use aws_sdk_s3::{
    Client,
    config::http::HttpResponse,
    error::SdkError,
    operation::{
        get_object::{GetObjectError, GetObjectOutput},
        put_object::{PutObjectError, PutObjectOutput},
    },
    primitives::ByteStream,
};

pub struct S3Client {
    client: Client,
    bucket: String,
    preview_bucket: String,
}

pub enum Position {
    Original,
    Preview,
}

impl S3Client {
    pub fn new(client: Client, bucket: String, preview_bucket: String) -> Self {
        Self {
            client,
            bucket,
            preview_bucket,
        }
    }

    fn position(&self, position: Position) -> &str {
        match position {
            Position::Original => &self.bucket,
            Position::Preview => &self.preview_bucket,
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
