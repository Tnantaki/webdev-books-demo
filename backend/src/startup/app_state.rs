use rust_decimal::dec;
use std::{fs, path::PathBuf};

use crate::{
   repos::in_mem::{books::BookRepo, images::ImageRepo},
   schemas::{book::AddBook, image::AddImage},
};

// in-memory storage (temporary)
#[derive(Clone)]
pub struct AppState {
   pub book_repo: BookRepo,
   pub image_repo: ImageRepo,
}

impl AppState {
   pub fn new() -> Self {
      let book_repo = BookRepo::new();
      let image_repo = ImageRepo::new();

      let img_path = mockup_image(&image_repo);
      mockup_books(&book_repo, &img_path);

      Self {
         book_repo,
         image_repo,
      }
   }
}

pub fn mockup_image(image_repo: &ImageRepo) -> String {
   let file_path = PathBuf::from("images").join("Yuki.jpg");
   let data = fs::read(&file_path).expect("read test file from images directory.");

   let mock_image = AddImage {
      filename: "Yuki".to_string(),
      content_type: "image/jpeg".to_string(),
      data: data.into(),
   };
   let id = image_repo.save_image(mock_image).unwrap();
   let img_path = format!("/images/{}", id);
   img_path
}

pub fn mockup_books(book_repo: &BookRepo, img_path: &str) {
   let mock_books = vec![
      AddBook {
         title: "C++".to_string(),
         genre: "Programming".to_string(),
         description: "OOP programming language".to_string(),
         price_in_pound: dec!(1_000.35),
         available: Some(0),
         img_path: img_path.to_string(),
      },
      AddBook {
         title: "Rust".to_string(),
         genre: "Programming".to_string(),
         description: "Secure programming language".to_string(),
         price_in_pound: dec!(1_300),
         available: Some(0),
         img_path: img_path.to_string(),
      },
   ];

   mock_books.into_iter().for_each(|book| {
      book_repo.add_book(book);
   });
}
