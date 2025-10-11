use crate::{models::images::ImageModel, repos::in_mem::InMemError, schemas::image::AddImage};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct ImageRepo {
   images: Arc<Mutex<Vec<ImageModel>>>,
}

impl ImageRepo {
   pub fn new() -> Self {
      Self {
         images: Arc::new(Mutex::new(vec![])),
      }
   }

   pub fn save_image(&self, new_image: AddImage) -> Result<Uuid, InMemError> {
      let image = ImageModel::add(new_image);
      let id = image.id;
      {
         let mut images = self.images.lock().map_err(|_| InMemError::LockPoisoned)?;
         images.push(image);
      }
      Ok(id)
   }

   pub fn get_image(&self, id: Uuid) -> Result<ImageModel, InMemError> {
      let images = self.images.lock().map_err(|_| InMemError::LockPoisoned)?;

      images
         .iter()
         .find(|book| book.id == id)
         .cloned()
         .ok_or(InMemError::DataNotFound("invalid image id".to_string()))
   }

   pub fn delete_image(&self, id: Uuid) -> Result<(), InMemError> {
      let mut images = self.images.lock().map_err(|_| InMemError::LockPoisoned)?;

      images
         .iter()
         .position(|book| book.id == id)
         .map(|idx| images.remove(idx))
         .map(|_| ())
         .ok_or(InMemError::DataNotFound("invalid image id".to_string()))
   }
}
