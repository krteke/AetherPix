use crate::{controllers::upload::UploadResult, models::_entities::images};

pub use super::_entities::images::{ActiveModel, Entity, Model};
use loco_rs::{model::ModelResult, prelude::*};
use sea_orm::{TransactionTrait, entity::prelude::*};
pub type Images = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn find_by_user_pid(db: &DatabaseConnection, pid: Uuid) -> ModelResult<Vec<Self>> {
        let images = images::Entity::find()
            .filter(
                model::query::condition()
                    .eq(images::Column::UserPid, pid)
                    .build(),
            )
            .all(db)
            .await?;

        Ok(images)
    }

    pub async fn find_by_filename(db: &DatabaseConnection, filename: &str) -> ModelResult<Self> {
        let images = images::Entity::find()
            .filter(
                model::query::condition()
                    .eq(images::Column::FileName, filename)
                    .eq(images::Column::Public, true)
                    .build(),
            )
            .one(db)
            .await?;

        images.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn create_with_upload_result(
        db: &DatabaseConnection,
        upload_result: &UploadResult,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;

        if images::Entity::find()
            .filter(
                model::query::condition()
                    .eq(images::Column::Url, &upload_result.url)
                    .build(),
            )
            .one(&txn)
            .await?
            .is_some()
        {
            return Err(ModelError::EntityAlreadyExists);
        }

        let image = images::ActiveModel {
            file_name: Set(upload_result.file_name.clone()),
            public: Set(upload_result.is_public),
            user_pid: Set(upload_result.user_id),
            url: Set(upload_result.url.clone()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;

        Ok(image)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
