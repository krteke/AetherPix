use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "images",
            &[
                ("id", ColType::PkAuto),
                ("url", ColType::Text),
                ("user_pid", ColType::UuidNull),
                ("public", ColType::Boolean),
                ("file_name", ColType::Text),
                ("uuid", ColType::Uuid),
                ("raw_name", ColType::Text),
                // ("size", ColType::String),
                (
                    "location",
                    ColType::Enum(
                        "location".to_string(),
                        vec!["local".to_string(), "r2".to_string()],
                    ),
                ),
            ],
            &[],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "images").await
    }
}
