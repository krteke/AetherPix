use async_trait::async_trait;
use aws_config::{Region, SdkConfig};
use aws_sdk_s3::{
    Client,
    config::{Credentials, SharedCredentialsProvider},
};
use axum::{Extension, Router};
use loco_rs::{
    Result,
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{BootResult, StartMode, create_app},
    config::Config,
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
};
use migration::Migrator;
use std::{path::Path, sync::Arc};

use crate::{common::settings::SettingsService, models::_entities::settings};
#[allow(unused_imports)]
use crate::{controllers, models::_entities::users, tasks, workers::downloader::DownloadWorker};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::auth::routes())
            .add_route(controllers::settings::routes())
            .add_route(controllers::upload::routes())
    }
    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::admin::Admin);
        // tasks-inject (do not remove)
    }
    async fn truncate(ctx: &AppContext) -> Result<()> {
        truncate_table(&ctx.db, users::Entity).await?;
        Ok(())
    }
    async fn seed(ctx: &AppContext, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(&ctx.db, &base.join("users.yaml").display().to_string())
            .await?;
        db::seed::<settings::ActiveModel>(
            &ctx.db,
            &base.join("settings.yaml").display().to_string(),
        )
        .await?;
        Ok(())
    }
    async fn after_routes(router: Router, ctx: &AppContext) -> Result<Router> {
        SettingsService::load(&ctx.db)
            .await
            .expect("加载系统配置失败");

        let s3_client = Arc::new(init_s3_client().await);

        Ok(router.layer(Extension(s3_client)))
    }
}

async fn init_s3_client() -> Client {
    let region = SettingsService::aws_region().await;
    let endpoint_url = SettingsService::aws_endpoint_url().await;
    let access_key_id = SettingsService::aws_access_key_id().await;
    let secret_access_key = SettingsService::aws_secret_access_key().await;

    let credentials = Credentials::builder()
        .access_key_id(access_key_id)
        .secret_access_key(secret_access_key)
        .build();

    let config = SdkConfig::builder()
        .region(Region::new(region))
        .endpoint_url(endpoint_url)
        .credentials_provider(SharedCredentialsProvider::new(credentials))
        .build();

    aws_sdk_s3::Client::new(&config)
}
