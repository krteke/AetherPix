use crate::models::_entities::tmps;

pub use super::_entities::tmps::{ActiveModel, Entity, Model};
use loco_rs::{
    model::{ModelError, ModelResult},
    prelude::model,
};
use sea_orm::entity::prelude::*;
pub type Tmps = Entity;

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
    pub async fn find_by_user_pid_and_file_name(
        db: &DatabaseConnection,
        user_pid: &str,
        file_name: &str,
    ) -> ModelResult<Self> {
        let parsed_uuid = uuid::Uuid::parse_str(user_pid).map_err(|e| ModelError::Any(e.into()))?;

        let item = tmps::Entity::find()
            .filter(
                model::query::condition()
                    .eq(tmps::Column::UserPid, parsed_uuid)
                    .eq(tmps::Column::FileName, file_name)
                    .build(),
            )
            .one(db)
            .await?;

        item.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn create_tmp_record(
        db: &DatabaseConnection,
        user_pid: uuid::Uuid,
        tmp_path: &str,
    ) -> ModelResult<Self> {
        todo!()
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
