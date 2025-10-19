use book_store::startup::command::Cli;
use clap::Parser;
use console::style;
use std::process;

#[tokio::main]
async fn main() {
   let cli = Cli::parse();

   if let Err(error) = book_store::run(cli).await {
      eprintln!("{} {}", style("error:").bold().red(), error);
      process::exit(1);
   }
}
