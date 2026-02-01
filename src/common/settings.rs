use std::{collections::HashMap, sync::LazyLock};

use loco_rs::Result;
use migration::OnConflict;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use serde_json::Value;
use tokio::sync::RwLock;

use crate::{
    common::settings::keys::DEFAULT_REGION, models::_entities::settings,
    views::settings::AppSettings,
};

static SETTINGS_CACHE: LazyLock<RwLock<HashMap<String, String>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub struct SettingsService;

mod keys {
    pub const UPLOAD_MAX_SIZE: &str = "upload_max_size_mb";
    pub const ALLOW_REGISTRATION: &str = "allow_registration";
    pub const SITE_NAME: &str = "site_name";
    pub const ALLOW_EVERYONE_UPLOAD: &str = "allow_everyone_upload";

    // secret, local garage
    pub const AWS_ACCESS_KEY_ID: &str = "aws_access_key_id";
    pub const AWS_SECRET_ACCESS_KEY: &str = "aws_secret_access_key";
    pub const AWS_REGION: &str = "aws_region";
    pub const AWS_ENDPOINT_URL: &str = "aws_endpoint_url";

    pub const DEFAULT_REGION: &str = "garage";
}

impl SettingsService {
    pub async fn load(db: &DatabaseConnection) -> Result<()> {
        let all_settings = settings::Entity::find().all(db).await?;

        let mut cache = SETTINGS_CACHE.write().await;
        for setting in all_settings {
            cache.insert(setting.key, setting.value);
        }
        tracing::info!("系统配置已加载");
        Ok(())
    }

    pub async fn max_upload_size() -> u64 {
        Self::get_u64(keys::UPLOAD_MAX_SIZE, 10).await
    }

    pub async fn allow_registration() -> bool {
        Self::get_bool(keys::ALLOW_REGISTRATION, false).await
    }

    pub async fn site_name() -> String {
        Self::get(keys::SITE_NAME, "AetherPix").await
    }

    pub async fn allow_everyone_upload() -> bool {
        Self::get_bool(keys::ALLOW_EVERYONE_UPLOAD, false).await
    }

    pub async fn aws_access_key_id() -> String {
        Self::get(keys::AWS_ACCESS_KEY_ID, "").await
    }

    pub async fn aws_secret_access_key() -> String {
        Self::get(keys::AWS_SECRET_ACCESS_KEY, "").await
    }

    pub async fn aws_region() -> String {
        Self::get(keys::AWS_REGION, DEFAULT_REGION).await
    }

    pub async fn aws_endpoint_url() -> String {
        Self::get(keys::AWS_ENDPOINT_URL, "").await
    }

    pub async fn get_app_settings() -> AppSettings {
        AppSettings {
            upload_max_size: Self::max_upload_size().await,
            allow_registration: Self::allow_registration().await,
            site_name: Self::site_name().await,
            allow_everyone_upload: Self::allow_everyone_upload().await,
        }
    }

    pub async fn update_batch(db: &DatabaseConnection, new_settings: &AppSettings) -> Result<()> {
        let json_value = serde_json::to_value(new_settings)?;
        let mut models = Vec::new();

        if let Value::Object(map) = json_value {
            for (k, v) in map {
                let val_str = match v {
                    Value::String(s) => s,
                    Value::Bool(b) => b.to_string(),
                    Value::Number(n) => n.to_string(),
                    _ => v.to_string(),
                };

                models.push(settings::ActiveModel {
                    key: Set(k),
                    value: Set(val_str),
                    ..Default::default()
                });
            }
        }

        settings::Entity::insert_many(models)
            .on_conflict(
                OnConflict::column(settings::Column::Key)
                    .update_column(settings::Column::Value)
                    .to_owned(),
            )
            .exec(db)
            .await?;

        Ok(())
    }

    async fn get(key: &str, default: &str) -> String {
        let cache = SETTINGS_CACHE.read().await;
        cache.get(key).cloned().unwrap_or(default.to_string())
    }

    async fn get_u64(key: &str, default: u64) -> u64 {
        Self::get(key, &default.to_string())
            .await
            .parse()
            .unwrap_or(default)
    }

    async fn get_bool(key: &str, default: bool) -> bool {
        Self::get(key, &default.to_string())
            .await
            .parse()
            .unwrap_or(default)
    }

    pub async fn set(db: &DatabaseConnection, key: &str, value: &str) -> Result<()> {
        let setting = settings::Entity::find()
            .filter(settings::Column::Key.eq(key))
            .one(db)
            .await?;

        let mut active_model = if let Some(s) = setting {
            s.into_active_model()
        } else {
            settings::ActiveModel {
                key: Set(key.to_string()),
                ..Default::default()
            }
        };

        active_model.value = Set(value.to_string());
        active_model.save(db).await?;

        let mut cache = SETTINGS_CACHE.write().await;
        cache.insert(key.to_string(), value.to_string());

        Ok(())
    }
}
