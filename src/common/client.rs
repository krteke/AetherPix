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
}

impl S3Client {
    pub fn new(client: Client, bucket: String) -> Self {
        Self { client, bucket }
    }

    pub async fn pub_object(
        &self,
        key: &str,
        body: ByteStream,
        content_type: &str,
    ) -> Result<PutObjectOutput, SdkError<PutObjectError, HttpResponse>> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body)
            .content_type(content_type)
            .send()
            .await
    }

    pub async fn get_object(
        &self,
        key: &str,
    ) -> Result<GetObjectOutput, SdkError<GetObjectError, HttpResponse>> {
        self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
    }
}
