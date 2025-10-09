use std::path::Path;
use crate::startup::ServerError;

pub struct Server {
   pub port: u16,
}

pub struct Config {
   pub server: Server,
   pub db_url: String,
   pub jwt_secret: String,
}

impl<'a> Config {
   pub fn build() -> Result<Config, ServerError<'static>> {
      dotenvy::from_path(Path::new(".env.sample"))
         .map_err(|_| ServerError::EnvError("Not found .env file"))?;

      let server = Server {
         port: dotenvy::var("SERVER_PORT")
            .map_err(|_| ServerError::EnvError("SERVER_PORT env not found"))?
            .parse()
            .map_err(|_| {
               ServerError::EnvError("invalid value, SERVER_PORT env must be number 1024 - 49151")
            })?,
      };

      let db_url = dotenvy::var("DATABASE_URL")
         .map_err(|_| ServerError::EnvError("DATABASE_URL env not found"))?;

      let jwt_secret = dotenvy::var("JWT_SECRET")
         .map_err(|_| ServerError::EnvError("JWT_SECRET env not found"))?;

      Ok(Config {
         server,
         db_url,
         jwt_secret,
      })
   }
}
