use sqlx::PgPool;
use uuid::Uuid;

use crate::{
   models::users::{Role, UserModel},
   routes::app_error::AppError,
   schemas::user::User,
};

#[derive(Clone)]
pub struct UserRepo {
   pool: PgPool,
}

impl UserRepo {
   pub fn new(pool: PgPool) -> Self {
      Self { pool }
   }

   pub async fn add_user(&self, email: String, password_hash: String) -> Result<(), AppError> {
      sqlx::query(
         r#"
            INSERT INTO users (email, password_hash, role)
            VALUES ($1, $2, $3)
         "#,
      )
      .bind(&email)
      .bind(&password_hash)
      .bind(&Role::User)
      .execute(&self.pool)
      .await?;

      Ok(())
   }
   
   pub async fn add_admin(&self, email: String, password_hash: String) -> Result<(), AppError> {
      sqlx::query(
         r#"
            INSERT INTO users (email, password_hash, role)
            VALUES ($1, $2, $3)
         "#,
      )
      .bind(&email)
      .bind(&password_hash)
      .bind(&Role::Admin)
      .execute(&self.pool)
      .await?;

      Ok(())
   }

   pub async fn get_user_by_email(&self, email: &str) -> Result<UserModel, AppError> {
      let user_model = sqlx::query_as::<_, UserModel>(
         r#"
            SELECT id, email, password_hash, role, created_at, updated_at
            FROM users WHERE email = $1
         "#,
      )
      .bind(email)
      .fetch_optional(&self.pool)
      .await?
      .ok_or(AppError::NotFound("invalid email".to_string()))?;

      Ok(user_model)
   }

   pub async fn get_user_by_id(&self, id: Uuid) -> Result<User, AppError> {
      let user = sqlx::query_as::<_, User>(
         r#"
            SELECT id, email, role, created_at, updated_at
            FROM users WHERE id = $1
         "#,
      )
      .bind(id)
      .fetch_optional(&self.pool)
      .await?
      .ok_or(AppError::NotFound("invalid user id".to_string()))?;

      Ok(user)
   }

   pub async fn edit_password(
      &self,
      id: Uuid,
      new_password: String,
   ) -> Result<UserModel, AppError> {
      let user_model = sqlx::query_as::<_, UserModel>(
         r#"
            UPDATE users SET
               password_hash = $1
            WHERE id = $2
            RETURNING
               id, email, password_hash, role, created_at, updated_at
         "#,
      )
      .bind(&new_password)
      .bind(id)
      .fetch_optional(&self.pool)
      .await?
      .ok_or(AppError::NotFound("invalid email".to_string()))?;

      Ok(user_model)
   }

   pub async fn get_all_user(&self) -> Result<Vec<User>, AppError> {
      let users = sqlx::query_as::<_, User>(
         r#"
            SELECT id, email, role, created_at, updated_at
            FROM users
         "#,
      )
      .fetch_all(&self.pool)
      .await?;

      if users.is_empty() {
         return Ok(vec![]);
      }

      Ok(users)
   }

   pub async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
      let row_affects = sqlx::query("DELETE FROM users WHERE id = $1")
         .bind(id)
         .execute(&self.pool)
         .await?
         .rows_affected();

      if row_affects == 0 {
         Err(AppError::NotFound("invalid user id".to_string()))
      } else {
         Ok(())
      }
   }
}
