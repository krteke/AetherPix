use std::sync::Arc;

use axum::extract::State;
use reqwest::StatusCode;

use crate::state::app::AppState;

pub async fn r2_url_handler(State(state): State<Arc<AppState>>) -> Result<String, StatusCode> {
    tracing::debug!("Base URL: {}", state.r2_url);

    Ok(state.r2_url.clone())
}
