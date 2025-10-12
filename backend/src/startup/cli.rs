use clap::{
   builder::{styling::AnsiColor, Styles}, Parser, Subcommand
};

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
}
