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
                ("email", ColType::StringUniq),
                ("username", ColType::StringUniq),
                (
                    "role",
                    ColType::EnumWithDefault(
                        "user_role".to_string(),
                        vec!["admin".to_string(), "user".to_string()],
                        "user".to_string(),
                    ),
                ),
                ("password", ColType::String),
                ("api_key", ColType::StringUniq),
                ("reset_token", ColType::StringNull),
                ("reset_sent_at", ColType::TimestampWithTimeZoneNull),
                ("email_verification_token", ColType::StringNull),
                (
                    "email_verification_sent_at",
                    ColType::TimestampWithTimeZoneNull,
                ),
                ("email_verified_at", ColType::TimestampWithTimeZoneNull),
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
