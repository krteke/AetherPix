use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "users",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::Uuid),
                ("username", ColType::StringUniq),
                ("password", ColType::String),
                ("api_key", ColType::StringUniq),
            ],
            &[],
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "users").await?;
        Ok(())
    }
}
