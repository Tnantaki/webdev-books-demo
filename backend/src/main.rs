use book_store::startup::{config, server};
use std::process;

#[tokio::main]
async fn main() {
   let config = config::Config::build().unwrap_or_else(|err| {
      eprintln!("Fail to get env: {}", err);
      process::exit(1);
   });

   server::run(config).await.unwrap_or_else(|err| {
      eprintln!("Fail to run server: {}", err);
      process::exit(1);
   });
}
