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
use std::fmt::Write;
use std::sync::Arc;
use tokio_util::io::ReaderStream;

use crate::{
    common::{
        client::{Position, S3Client},
        settings::SettingsService,
    },
    models::{_entities::images, users::users},
    views::view::{Image, ListViewResponse},
};

const MAX_PAGE_SIZE: u64 = 20;

#[derive(Deserialize)]
pub struct ListViewParams {
    pub page: u64,
    pub limit: u64,
}

async fn view(
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<Arc<S3Client>>,
    headers: HeaderMap,
    Path(name): Path<String>,
) -> Result<Response> {
    images::Model::find_by_filename(&ctx.db, &name).await?;

    fetch_file(headers, &s3_client, &name, Position::Webp).await
}

async fn preview(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<Arc<S3Client>>,
    headers: HeaderMap,
    Path(name): Path<String>,
) -> Result<Response> {
    let user_pid = auth.claims.pid;
    let user = users::Model::find_by_pid(&ctx.db, &user_pid).await?;

    let uuid = &name[0..name.len() - 5];

    if let Err(e) = images::Model::find_by_uuid_and_pid(&ctx.db, user.pid, uuid).await {
        tracing::error!("Failed to find image by UUID and PID: {}", e);
        return Err(Error::NotFound);
    }

    fetch_file(headers, &s3_client, &name, Position::Preview).await
}

async fn list(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(params): Query<ListViewParams>,
) -> Result<Response> {
    let user_pid = auth.claims.pid;
    let user = users::Model::find_by_pid(&ctx.db, &user_pid).await?;

    let page_size = params.limit.min(MAX_PAGE_SIZE);
    let (images, num_items_and_pages) =
        images::Model::find_by_user_pid(&ctx.db, user.pid, params.page, page_size).await?;

    let local_base_url = SettingsService::local_base_url().await;
    let base_url = if local_base_url.trim().is_empty() {
        ctx.config.server.full_url() + "/api/view/preview"
    } else {
        local_base_url + "/preview"
    };

    let images: Vec<Image> = images
        .into_iter()
        .map(|m| {
            let mut url = String::with_capacity(base_url.len() + 41);
            let _ = write!(url, "{}/{}.webp", base_url, m.uuid);

            Image {
                preview_url: url,
                original_url: m.url,
                name: m.raw_name,
                size: m.size,
                id: m.id,
            }
        })
        .collect();

    format::json(ListViewResponse {
        images,
        pages: num_items_and_pages.number_of_pages,
        total: num_items_and_pages.number_of_items,
    })
}

async fn fetch_file(
    headers: HeaderMap,
    s3_client: &S3Client,
    name: &str,
    position: Position,
) -> Result<Response> {
    let if_none_match = headers.get(IF_NONE_MATCH).and_then(|h| h.to_str().ok());

    let s3_req = s3_client.get_object(name, position).await;
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

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/view")
        .add("/{name}", get(view))
        .add("/preview/{name}", get(preview))
        .add("/list", get(list))
}
