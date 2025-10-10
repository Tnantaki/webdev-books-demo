use crate::{models::images::ImageModel, schemas::image::AddImage};
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

   pub fn save_image(&self, new_image: AddImage) -> Option<Uuid> {
      let image = ImageModel::add(new_image);
      let id = image.id;
      {
         let mut images = self.images.lock().unwrap();
         images.push(image);
      }
      Some(id)
   }

   pub fn get_image(&self, id: Uuid) -> Option<ImageModel> {
      let images = self.images.lock().unwrap();

      images.iter().find(|book| book.id == id).cloned()
   }

   pub fn delete_image(&self, id: Uuid) -> Option<()> {
      let mut images = self.images.lock().unwrap();

      images.iter().position(|book| book.id == id).map(|idx| images.remove(idx)).map(|_| ())
   }
}
