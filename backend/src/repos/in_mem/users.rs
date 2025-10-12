use crate::{
   models::users::{Role, UserModel},
   repos::in_mem::InMemError,
};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepo {
   users: Arc<Mutex<Vec<UserModel>>>,
}

impl UserRepo {
   pub fn new() -> Self {
      Self {
         users: Arc::new(Mutex::new(vec![])),
      }
   }

   pub fn add_user(
      &self,
      email: String,
      password_hash: String,
      role: Role,
   ) -> Result<(), InMemError> {
      let user = UserModel::add_user(email, password_hash, role);
      {
         let mut users = self.users.lock().map_err(|_| InMemError::LockPoisoned)?;
         users.push(user);
      }
      Ok(())
   }

   pub fn get_users(&self) -> Result<Vec<UserModel>, InMemError> {
      let users = self.users.lock().map_err(|_| InMemError::LockPoisoned)?.clone();

      Ok(users)
   }

   pub fn get_user_by_id(&self, id: Uuid) -> Result<UserModel, InMemError> {
      let users = self.users.lock().map_err(|_| InMemError::LockPoisoned)?;

      users
         .iter()
         .find(|user| user.id == id)
         .cloned()
         .ok_or(InMemError::DataNotFound("invalid user id".to_string()))
   }

   pub fn get_user_by_email(&self, email: &str) -> Result<UserModel, InMemError> {
      let users = self.users.lock().map_err(|_| InMemError::LockPoisoned)?;

      users
         .iter()
         .find(|user| user.email == email)
         .cloned()
         .ok_or(InMemError::DataNotFound("invalid email".to_string()))
   }

   pub fn edit_password(&self, id: Uuid, new_password: String) -> Result<UserModel, InMemError> {
      let mut users = self.users.lock().map_err(|_| InMemError::LockPoisoned)?;

      users
         .iter_mut()
         .find(|user| user.id == id)
         .map(|user| {
            user.password_hash = new_password;
            return user.clone();
         })
         .ok_or(InMemError::DataNotFound("invalid user id".to_string()))
   }

   pub fn delete_user(&self, id: Uuid) -> Result<(), InMemError> {
      let mut users = self.users.lock().map_err(|_| InMemError::LockPoisoned)?;

      users
         .iter()
         .position(|user| user.id == id)
         .map(|idx| users.remove(idx))
         .map(|_| ())
         .ok_or(InMemError::DataNotFound("invalid user id".to_string()))
   }
}
