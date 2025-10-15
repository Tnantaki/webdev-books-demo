use crate::{
   models::users::Role,
   repos::in_mem::{books::BookRepo, images::ImageRepo, users::UserRepo},
   schemas::{book::AddBook, image::AddImage},
   services::password_hashing::PasswordService,
};
use rust_decimal::dec;
use std::{fs, path::PathBuf};
use uuid::Uuid;

pub fn mockup_admin(user_repo: &UserRepo) {
   let email = "admin@email.com".to_string();
   let password_hash = PasswordService::hash("Admin1234").unwrap();
   user_repo.add_user(email, password_hash, Role::Admin).unwrap();
}

pub fn mockup_image(image_repo: &ImageRepo) -> Uuid {
   let file_path = PathBuf::from("images").join("Yuki.jpg");
   let data = fs::read(&file_path).expect("read test file from images directory.");

   let mock_image = AddImage {
      filename: "Yuki".to_string(),
      content_type: "image/jpeg".to_string(),
      data,
   };
   let id = image_repo.save_image(mock_image).unwrap();
   id
}

pub fn mockup_books(book_repo: &BookRepo, image_id: Uuid) {
   let mock_books = vec![
      AddBook {
         title: "C++".to_string(),
         genre: "Programming".to_string(),
         description: "OOP programming language".to_string(),
         price_in_pound: dec!(1_000.35),
         available: Some(0),
         image_id,
      },
      AddBook {
         title: "Rust".to_string(),
         genre: "Programming".to_string(),
         description: "Secure programming language".to_string(),
         price_in_pound: dec!(1_300),
         available: Some(0),
         image_id,
      },
   ];

   mock_books.into_iter().for_each(|book| {
      book_repo.add_book(book).unwrap();
   });
}
