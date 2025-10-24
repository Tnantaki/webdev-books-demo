use crate::{
   ServerError,
   repos::postgres::users::UserRepo,
   schemas::admin::{Email, Password},
   services::password_hashing::PasswordService,
   startup::command::display_error,
};
use console::style;
use dialoguer::{Input, Password as PasswordInput};
use sqlx::{Pool, Postgres};

pub async fn run(pool: Pool<Postgres>) -> Result<(), ServerError> {
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
               display_error("Email already exists");
               continue;
            }
            break email.value().to_string();
         }
         Err(error) => {
            display_error(&error);
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
            display_error(&error);
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
      display_error("Password do not match");
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
