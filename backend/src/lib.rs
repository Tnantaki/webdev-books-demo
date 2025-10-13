pub mod models;
pub mod repos;
pub mod routes;
pub mod schemas;
pub mod services;
pub mod startup;

use crate::startup::{
   cli::{Cli, Commands, create_admin},
   config, server,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError<'a> {
   #[error("Fail to get env: {0}")]
   EnvError(&'a str),

   #[error("Fail to run server: {0}")]
   RunServerError(String),

   #[error("Fail to connect to database: {0}")]
   DatabaseError(String),

   #[error("Fail to create admin: {0}")]
   CreateAdminError(String),
}

pub async fn run(cli: Cli) -> Result<(), ServerError<'static>> {
   let config = config::Config::build()?;

   let pool = repos::postgres::connect(&config.db_url).await?;
   println!("Connect to postgres database success.");

   match cli.command {
      Some(Commands::Serve) => {
         server::run(config, pool).await?;
      }
      Some(Commands::CreateAdmin) => {
         // println!("Create Admin as user input..."); // TODO: implement add admin account logic
         create_admin(pool).await?;
      }
      None => {
         server::run(config, pool).await?;
      }
   }

   Ok(())
}
