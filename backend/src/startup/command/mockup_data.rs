use console::style;
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::{
   fs,
   io::{self, Write},
   path::PathBuf,
};
use uuid::Uuid;

use crate::{
   ServerError,
   repos::postgres::{books::BookRepo, images::ImageRepo},
   schemas::{
      book::AddBook,
      image::{AddImage, ImgType},
   },
};

#[derive(Debug, Deserialize)]
struct MockupBook {
   // #[serde(skip)]
   upc: String,
   title: String,
   genre: String,
   description: String,
   avaiable: i32,
   #[serde(rename = "priceInPound")]
   price_in_pound: Decimal,
   #[allow(unused)]
   rating: i16,
   #[serde(rename = "imgPath")]
   img_path: String,
}

// !warning, This function read data in all file to memory before push to database
// If total files size is large. modify it to read file by file.
pub async fn propagate_books(pool: Pool<Postgres>) -> Result<(), ServerError<'static>> {
   let mut img_ids: Vec<Uuid> = vec![];

   // Read JSON file
   let json_data = fs::read_to_string("./mockup/book_details.json")
      .map_err(|e| ServerError::SeedDataError(e.to_string()))?;

   let mut books: Vec<MockupBook> =
      serde_json::from_str(&json_data).map_err(|e| ServerError::SeedDataError(e.to_string()))?;

   display_process(&format!("Deserialize JSON found {} books", books.len()));

   books.truncate(10); // DEBUG: test

   // Read all image files to memory
   let mut image_datas: Vec<(String, ImgType, Vec<u8>)> = vec![];
   for (idx, book) in books.iter().enumerate() {
      let file_path = PathBuf::from("mockup").join(&book.img_path);
      match fs::read(file_path) {
         Ok(data) => {
            let extension = book.img_path.rsplit('.').next().ok_or(ServerError::SeedDataError(
               "Invalid file extension.".to_string(),
            ))?;

            // Validate and convert to content type
            let img_type = ImgType::from_extension(extension)
               .map_err(|e| ServerError::DatabaseError(e.to_string()))?;
            image_datas.push((book.upc.clone(), img_type, data));
            progress_indicator(&format!("Reading image file number: {}", idx + 1));
         }
         Err(err) => return Err(ServerError::SeedDataError(err.to_string())),
      }
   }
   println!("");

   let image_repo = ImageRepo::new(pool.clone());
   for (idx, img) in image_datas.into_iter().enumerate() {
      let (filename, img_type, data) = img;
      let add_image = AddImage::new(filename, img_type, data);
      let img_meta = image_repo
         .save_image(add_image)
         .await
         .map_err(|e| ServerError::SeedDataError(e.to_string()))?;

      img_ids.push(img_meta.id);
      progress_indicator(&format!("Save image to database number: {}", idx + 1));
   }
   println!("");

   let book_repo = BookRepo::new(pool);

   if books.len() != img_ids.len() {
      return Err(ServerError::SeedDataError(
         "Incomplete propagate image.".to_string(),
      ));
   }

   for (idx, book) in books.into_iter().enumerate() {
      let add_book = AddBook {
         title: book.title,
         genre: book.genre,
         description: book.description,
         price_in_pound: book.price_in_pound,
         image_id: img_ids[idx],
         available: Some(book.avaiable),
      };

      book_repo
         .add_book(add_book)
         .await
         .map_err(|e| ServerError::SeedDataError(e.to_string()))?;

      progress_indicator(&format!("Save book to database number: {}", idx + 1));
   }
   println!("");

   println!(
      "{}",
      style("\n✅ Propagate books to database successfully!").green()
   );
   Ok(())
}

fn display_process(msg: &str) {
   println!("{} {}", style("process:").bold().green(), msg);
}

fn progress_indicator(msg: &str) {
   print!("\r{} {}", style("progress:").bold().magenta(), msg);
   io::stdout().flush().unwrap();
}
