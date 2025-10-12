use crate::ServerError;
use std::path::Path;

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
            .map_err(|_| {
               ServerError::EnvError("`SERVER_PORT` environment variable must be provided")
            })?
            .parse()
            .map_err(|_| {
               ServerError::EnvError(
                  "invalid value, `SERVER_PORT` environment variable must be number 1024 - 49151",
               )
            })?,
      };

      let db_url = dotenvy::var("DATABASE_URL").map_err(|_| {
         ServerError::EnvError("`DATABASE_URL` environment variable must be provided")
      })?;

      let jwt_secret = dotenvy::var("JWT_SECRET").map_err(|_| {
         ServerError::EnvError("`JWT_SECRET` environment variable must be provided")
      })?;

      Ok(Config {
         server,
         db_url,
         jwt_secret,
      })
   }
}
