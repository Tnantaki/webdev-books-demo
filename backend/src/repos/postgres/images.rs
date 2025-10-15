use sqlx::PgPool;
use uuid::Uuid;

use crate::{
   models::images::{ImageMetadata, ImageModel},
   routes::app_error::AppError,
   schemas::image::AddImage,
};

#[derive(Clone)]
pub struct ImageRepo {
   pool: PgPool,
}

impl ImageRepo {
   pub fn new(pool: PgPool) -> Self {
      Self { pool }
   }

   pub async fn save_image(&self, image: AddImage) -> Result<ImageMetadata, AppError> {
      let file_size = image.data.len() as i32;

      let image_metadata = sqlx::query_as::<_, ImageMetadata>(
         r#"
            INSERT INTO images (filename, content_type, data, file_size)
            VALUES ($1, $2, $3, $4)
            RETURNING id, filename, content_type, file_size, created_at
         "#,
      )
      .bind(&image.filename)
      .bind(&image.content_type)
      .bind(&image.data)
      .bind(file_size)
      .fetch_one(&self.pool)
      .await?;

      Ok(image_metadata)
   }

   pub async fn get_image_by_id(&self, id: Uuid) -> Result<ImageModel, AppError> {
      let image = sqlx::query_as::<_, ImageModel>(
         r#"
            SELECT id, filename, content_type, data, file_size, created_at
            FROM images WHERE id = $1
         "#,
      )
      .bind(id)
      .fetch_optional(&self.pool)
      .await?
      .ok_or(AppError::NotFound("invalid image id".to_string()))?;

      Ok(image)
   }

   pub async fn delete_image(&self, id: Uuid) -> Result<(), AppError> {
      let row_affects = sqlx::query("DELETE FROM images WHERE id = $1")
         .bind(id)
         .execute(&self.pool)
         .await?
         .rows_affected();

      if row_affects == 0 {
         Err(AppError::NotFound("invalid image id".to_string()))
      } else {
         Ok(())
      }
   }

   pub async fn cleanup_orphan_images(&self, min_age_hours: i32) -> Result<(), sqlx::Error> {
      let orphan_image_ids = self.find_orphan_image_ids(min_age_hours).await?;

      println!("Found {} orphan images", orphan_image_ids.len());

      for id in orphan_image_ids {
         match self.delete_image(id).await {
            Ok(_) => println!("Deleted orphan image id: {} ", id),
            Err(e) => eprintln!("Failed to delete {}: {}", id, e),
         }
      }

      Ok(())
   }

   async fn find_orphan_image_ids(&self, min_age_hours: i32) -> Result<Vec<Uuid>, sqlx::Error> {
      let image_ids: Vec<Uuid> = sqlx::query_scalar(
         r#"
            SELECT i.id
            FROM images i
            WHERE NOT EXISTS (
               SELECT 1 FROM books b WHERE b.image_id = i.id
            )
            AND i.created_at < NOW() - make_interval(hours => $1)
            ORDER BY i.created_at ASC
            LIMIT 100
         "#,
      )
      .bind(min_age_hours)
      .fetch_all(&self.pool)
      .await?;

      if image_ids.is_empty() {
         return Ok(vec![]);
      }
      Ok(image_ids)
   }
}
