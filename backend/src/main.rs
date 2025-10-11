use book_store::{
   repos,
   startup::{config, server},
};
use std::process;

#[tokio::main]
async fn main() {
   let config = config::Config::build().unwrap_or_else(|err| {
      eprintln!("Fail to get env: {}", err);
      process::exit(1);
   });

   let pool = repos::postgres::connect(&config.db_url).await.unwrap_or_else(|err| {
      eprintln!("Fail to connect to database: {}", err);
      process::exit(1);
   });
   println!("Connect to postgres database success.");

   server::run(config, pool).await.unwrap_or_else(|err| {
      eprintln!("Fail to run server: {}", err);
      process::exit(1);
   });
}
