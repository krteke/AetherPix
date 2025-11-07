use std::{collections::HashMap, sync::Arc, time::Duration};

use aws_sdk_s3::presigning::PresigningConfig;
use axum::{extract::State, Json};
use reqwest::StatusCode;

use crate::{
    models::{PresignRequest, PresignedResponse},
    state::app::AppState,
};

pub async fn presign_upload_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<Vec<PresignRequest>>,
) -> Result<Json<PresignedResponse>, (StatusCode, String)> {
    let expiration = Duration::from_secs(60 * 10);

    let presigning_config = PresigningConfig::expires_in(expiration).map_err(|e| {
        tracing::error!("Error creating presigning config: {}", e);

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create presigning config.".to_string(),
        )
    })?;

    let mut urls = HashMap::new();

    for unique_key in req {
        let final_r2_key = format!("uploads/{}.{}", unique_key.uuid, unique_key.extension);

        match state
            .r2_client
            .put_object()
            .bucket(&state.r2_bucket)
            .key(&final_r2_key)
            .presigned(presigning_config.clone())
            .await
        {
            Ok(presigned_req) => {
                urls.insert(unique_key.uuid, presigned_req.uri().to_string());
            }
            Err(e) => {
                tracing::error!("Error presigning upload: {}", e);

                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error presigning upload.".to_string(),
                ));
            }
        }
    }

    Ok(Json(PresignedResponse { urls }))
}
