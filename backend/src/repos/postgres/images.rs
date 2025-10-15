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
}

// async fn find_orphan_images(pool: &PgPool) -> Result<Vec<Uuid>, sqlx::Error> {
//    let image_ids = sqlx::query_scalar::<Uuid>(
//       r#"
//          SELECT i.id
//          FROM images i
//          JOIN books b ON b.image_id = i.id
//       "#
//    )
// }