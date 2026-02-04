use std::{io::Cursor, path::PathBuf};

use aws_sdk_s3::primitives::ByteStream;
use image::ImageReader;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{common::client::get_garage, controllers::upload::TempFileGuard};

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    // pub file_path: String,
    pub tmp_file_guard: TempFileGuard,
    pub preview_key: String,
}

#[async_trait]
impl BackgroundWorker<WorkerArgs> for Worker {
    /// Creates a new instance of the Worker with the given application context.
    ///
    /// This function is called when registering the worker with the queue system.
    ///
    /// # Parameters
    /// * `ctx` - The application context containing shared resources
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    /// Returns the class name of the worker.
    ///
    /// This name is used when enqueueing jobs and identifying the worker in logs.
    /// The implementation returns the struct name as a string.
    fn class_name() -> String {
        "Thumbnail".to_string()
    }

    /// Returns tags associated with this worker.
    ///
    /// Tags can be used to filter which workers run during startup.
    /// The default implementation returns an empty vector (no tags).
    fn tags() -> Vec<String> {
        Vec::new()
    }

    /// Performs the actual work when a job is processed.
    ///
    /// This is the main function that contains the worker's logic.
    /// It gets executed when a job is dequeued from the job queue.
    ///
    /// # Returns
    /// * `Result<()>` - Ok if the job completed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================Thumbnail=======================");
        let client = get_garage();
        let tmp_file_path = args.tmp_file_guard.0.clone();
        let preview_name = args.preview_key;

        let result = async move {
            let thumbnail_data =
                tokio::task::spawn_blocking(move || process_thumbnail(tmp_file_path))
                    .await
                    .map_err(|e| e.to_string())?
                    .map_err(|e| e.to_string())?;
            let body = ByteStream::from(thumbnail_data);

            client
                .pub_object(
                    &preview_name,
                    body,
                    "image/webp",
                    crate::common::client::Position::Preview,
                )
                .await
                .map_err(|e| e.to_string())
        };

        drop(args.tmp_file_guard);

        match result.await {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("Failed to upload thumbnail: {}", e);
                Err(Error::InternalServerError)
            }
        }
    }
}

fn process_thumbnail(file_path: PathBuf) -> Result<Vec<u8>, String> {
    let img = ImageReader::open(file_path)
        .map_err(|e| e.to_string())?
        .with_guessed_format()
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    let thumbnail = img.resize(400, 400, image::imageops::FilterType::Triangle);

    let mut buffer = Cursor::new(Vec::new());
    thumbnail
        .write_to(&mut buffer, image::ImageFormat::WebP)
        .map_err(|e| e.to_string())?;

    Ok(buffer.into_inner())
}
