#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20260201_113639_settings;
mod m20260202_143532_images;
mod m20260206_114421_tmps;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20260201_113639_settings::Migration),
            Box::new(m20260202_143532_images::Migration),
            Box::new(m20260206_114421_tmps::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}