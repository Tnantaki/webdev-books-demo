pub mod clean;
pub mod create_admin;
pub mod seed;

use clap::{
   Parser, Subcommand,
   builder::{Styles, styling::AnsiColor},
};
use console::style;

const HELP_STYLES: Styles = Styles::styled()
   .header(AnsiColor::Blue.on_default().bold())
   .usage(AnsiColor::Blue.on_default().bold())
   .literal(AnsiColor::White.on_default())
   .placeholder(AnsiColor::Green.on_default());

// get default from Cargo.toml file if didn't specify
// version: version
// about: description
#[derive(Parser)]
#[command(version, about, styles = HELP_STYLES)]
pub struct Cli {
   #[command(subcommand)]
   pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
   /// Start the web server
   Serve,
   /// Create an admin user interactively
   CreateAdmin,
   /// Database Management
   Database {
      #[command(subcommand)]
      command: DatabaseCommands,
   },
}

#[derive(Subcommand)]
pub enum DatabaseCommands {
   /// Mock up book data from mockup directory
   Seed,
   /// Empty all tables in database
   Clean,
}

pub fn display_error(error: &str) {
   println!("{} {}", style("error:").bold().red(), style(error).red());
}
