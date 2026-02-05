use loco_rs::prelude::*;
use serde::Deserialize;

use crate::{
    common::settings::SettingsService,
    models::users::users::{self, UserRole},
    views::settings::AppSettings,
};

#[derive(Deserialize)]
struct SettingParams {
    name: String,
    value: String,
}

async fn settings() -> Result<Response> {
    let settings = SettingsService::get_app_settings().await;

    format::json(settings)
}

async fn update_settings(
    jwt: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<AppSettings>,
) -> Result<Response> {
    if let Ok(user) = users::Model::find_by_pid(&ctx.db, &jwt.claims.pid).await
        && user.role == UserRole::Admin
    {
        SettingsService::update_batch(&ctx.db, &params).await?;
    }

    format::json(())
}

async fn update(
    jwt: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<SettingParams>,
) -> Result<Response> {
    if let Ok(user) = users::Model::find_by_pid(&ctx.db, &jwt.claims.pid).await
        && user.role == UserRole::Admin
    {
        SettingsService::set(&ctx.db, &params.name, &params.value).await?;
    }

    format::json(())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/settings", get(settings))
        .add("/settings/update", post(update_settings))
        .add("/setting/update", post(update))
}
