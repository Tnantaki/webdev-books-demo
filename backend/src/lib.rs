pub mod models;
pub mod repos;
pub mod routes;
pub mod schemas;
pub mod services;
pub mod startup;

use crate::startup::{
   command::{Cli, Commands, DatabaseCommands, clean, create_admin, seed},
   config::{self, ConfigError},
   server,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
   #[error("Fail to get env: {0}")]
   EnvError(String),

   #[error("Fail to run server: {0}")]
   RunServerError(String),

   #[error("Fail to connect to database: {0}")]
   DatabaseError(String),

   #[error("Fail to create admin: {0}")]
   CreateAdminError(String),

   #[error("Fail to mockup data: {0}")]
   SeedDataError(String),

   #[error("Fail to clean database: {0}")]
   CleanDataError(String),
}

impl From<ConfigError> for ServerError {
   fn from(error: ConfigError) -> Self {
      ServerError::EnvError(error.to_string())
   }
}

pub async fn run(cli: Cli) -> Result<(), ServerError> {
   let config = config::Config::build()?;

   let pool = repos::postgres::connect(&config.db_url).await?;
   println!("Connect to postgres database success.");

   match cli.command {
      Some(Commands::Serve) => {
         server::run(config, pool).await?;
      }
      Some(Commands::CreateAdmin) => {
         create_admin::run(pool).await?;
      }
      Some(Commands::Database { command }) => match command {
         DatabaseCommands::Seed => {
            seed::propagate_books(pool).await?;
         }
         DatabaseCommands::Clean => {
            clean::empty_all_tables(pool).await?;
         }
      },
      None => {
         server::run(config, pool).await?;
      }
   }

   Ok(())
}
