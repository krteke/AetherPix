use crate::{
    controllers::upload::UploadResult,
    models::_entities::images::{self, Location},
};

pub use super::_entities::images::{ActiveModel, Entity, Model};
use loco_rs::{model::ModelResult, prelude::*};
use sea_orm::{ItemsAndPagesNumber, QueryOrder, TransactionTrait, entity::prelude::*};
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
    pub async fn find_by_user_pid(
        db: &DatabaseConnection,
        pid: Uuid,
        page: u64,
        page_size: u64,
        location: Option<Location>,
    ) -> ModelResult<(Vec<Self>, ItemsAndPagesNumber)> {
        let mut filter = model::query::condition();
        if let Some(loc) = location {
            filter = filter.eq(images::Column::Location, loc);
        }
        filter = filter.eq(images::Column::UserPid, Some(pid));

        let query = images::Entity::find()
            .filter(filter.build())
            .order_by_desc(images::Column::Id)
            .paginate(db, page_size);
        let num_items_and_pages = query.num_items_and_pages().await?;
        let page = page.min(num_items_and_pages.number_of_pages);

        let images = query.fetch_page(page).await?;

        Ok((images, num_items_and_pages))
    }

    pub async fn find_by_uuid_and_pid(
        db: &DatabaseConnection,
        pid: Uuid,
        uuid: &str,
        location: Option<Location>,
    ) -> ModelResult<Model> {
        let parse_uuid = Uuid::parse_str(uuid).map_err(|e| ModelError::Any(e.into()))?;

        let mut filter = model::query::condition();
        if let Some(loc) = location {
            filter = filter.eq(images::Column::Location, loc);
        }
        filter = filter
            .eq(images::Column::Uuid, parse_uuid)
            .eq(images::Column::UserPid, pid);

        let image = images::Entity::find()
            .filter(filter.build())
            .one(db)
            .await?;

        image.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn find_by_filename(
        db: &DatabaseConnection,
        filename: &str,
        location: Option<Location>,
    ) -> ModelResult<Self> {
        let mut filter = model::query::condition();
        if let Some(loc) = location {
            filter = filter.eq(images::Column::Location, loc);
        }
        filter = filter
            .eq(images::Column::FileName, filename)
            .eq(images::Column::Public, true);

        let images = images::Entity::find()
            .filter(filter.build())
            .one(db)
            .await?;

        images.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn save_local_with_result(
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
            uuid: Set(upload_result.uuid),
            raw_name: Set(upload_result.raw_name.clone()),
            // size: Set(upload_result.size.clone()),
            location: Set(images::Location::Local),
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
