use crate::{
   ServerError,
   repos::postgres::users::UserRepo,
   schemas::admin::{Email, Password},
   services::password_hashing::PasswordService,
};
use clap::{
   Parser, Subcommand,
   builder::{Styles, styling::AnsiColor},
};
use console::style;
use dialoguer::{Input, Password as PasswordInput};
use sqlx::{Pool, Postgres};

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

pub async fn create_admin(pool: Pool<Postgres>) -> Result<(), ServerError<'static>> {
   let user_repo = UserRepo::new(pool);

   println!("\n🔧 Create Admin User");

   let email: String = loop {
      let input: String = Input::new()
         .with_prompt("email")
         .interact_text()
         .map_err(|e| ServerError::CreateAdminError(e.to_string()))?;

      match Email::new(input) {
         Ok(email) => {
            if user_repo.get_user_by_email(email.value()).await.is_ok() {
               diaplay_error("Email already exists");
               continue;
            }
            break email.value().to_string();
         }
         Err(error) => {
            diaplay_error(&error);
         }
      };
   };

   let password: Password = loop {
      let pass_input: String = PasswordInput::new()
         .with_prompt("password")
         .interact()
         .map_err(|e| ServerError::CreateAdminError(e.to_string()))?;

      match Password::new(pass_input) {
         Ok(password) => break password,
         Err(error) => {
            diaplay_error(&error);
         }
      }
   };

   loop {
      let pass_again: String = PasswordInput::new()
         .with_prompt("password (again)")
         .interact()
         .map_err(|e| ServerError::CreateAdminError(e.to_string()))?;

      if password.value() == pass_again {
         break;
      }
      diaplay_error("Password do not match");
   }

   let password_hash = PasswordService::hash(&password.value())
      .map_err(|e| ServerError::CreateAdminError(e.to_string()))?;

   user_repo
      .add_admin(email, password_hash)
      .await
      .map_err(|e| ServerError::CreateAdminError(e.to_string()))?;

   println!("{}", style("\n✅ Admin user created successfully!").green());

   Ok(())
}

fn diaplay_error(error: &str) {
   println!("{} {}", style("error:").bold().red(), style(error).red());
}
