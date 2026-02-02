use std::sync::Arc;

use axum::{
    Extension,
    body::Body,
    http::{
        HeaderMap, StatusCode,
        header::{self, IF_NONE_MATCH},
    },
};
use loco_rs::prelude::*;
use serde::Deserialize;
use tokio_util::io::ReaderStream;

use crate::{common::client::S3Client, models::_entities::images};

#[derive(Deserialize)]
pub struct ListViewParams {
    pub page: u32,
    pub limit: u32,
}

async fn view(
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<Arc<S3Client>>,
    headers: HeaderMap,
    Path(name): Path<String>,
) -> Result<Response> {
    images::Model::find_by_filename(&ctx.db, &name).await?;

    let if_none_match = headers.get(IF_NONE_MATCH).and_then(|h| h.to_str().ok());

    let s3_req = s3_client.get_object(&name).await;
    let output = match s3_req {
        Ok(o) => o,
        Err(e) => {
            tracing::error!("Error getting object from S3: {}", e);
            if e.into_service_error().is_no_such_key() {
                return Err(Error::NotFound);
            }
            return Err(Error::InternalServerError);
        }
    };

    let mut response = Response::builder().status(StatusCode::OK);

    if let Some(etag) = output.e_tag() {
        if let Some(client_etag) = if_none_match
            && client_etag == etag
        {
            tracing::debug!("Etag matched, returning NOT_MODIFIED");
            return Ok(StatusCode::NOT_MODIFIED.into_response());
        }
        response = response.header(header::ETAG, etag)
    }
    if let Some(content_type) = output.content_type() {
        response = response.header(header::CONTENT_TYPE, content_type)
    }
    if let Some(len) = output.content_length() {
        response = response.header(header::CONTENT_LENGTH, len)
    }

    response = response.header(header::CACHE_CONTROL, "public, max-age=2592000, immutable");
    let async_read = output.body.into_async_read();
    let reader_stream = ReaderStream::new(async_read);
    let body = Body::from_stream(reader_stream);

    let response = response.body(body)?;

    Ok(response)
}

async fn view_list(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<ListViewParams>,
) -> Result<Response> {
    todo!()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/view")
        .add("/{name}", get(view))
        .add("/list", get(view_list))
}
