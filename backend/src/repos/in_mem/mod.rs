use crate::repos::in_mem::{
   books::BookRepo,
   images::ImageRepo,
   mockup::{mockup_admin, mockup_books, mockup_image},
   users::UserRepo,
};
use thiserror::Error;

pub mod books;
pub mod images;
pub mod mockup;
pub mod users;

#[derive(Error, Debug)]
pub enum InMemError {
   #[error("{0}")]
   DataNotFound(String),

   #[error("Lock poisoned")]
   LockPoisoned,
}

// in-memory storage (temporary)
#[derive(Clone)]
pub struct InMemRepos {
   pub book_repo: BookRepo,
   pub image_repo: ImageRepo,
   pub user_repo: UserRepo,
}

impl InMemRepos {
   pub fn new() -> Self {
      let book_repo = BookRepo::new();
      let image_repo = ImageRepo::new();
      let user_repo = UserRepo::new();

      mockup_admin(&user_repo);
      let img_path = mockup_image(&image_repo);
      mockup_books(&book_repo, &img_path);

      Self {
         book_repo,
         image_repo,
         user_repo,
      }
   }
}
