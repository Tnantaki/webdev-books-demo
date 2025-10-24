use std::path::Path;
use thiserror::Error;

const SERVER_PORT_KEY: &'static str = "SERVER_PORT";

#[derive(Error, Debug)]
pub enum ConfigError {
   #[error("{0}")]
   DotEnvyError(String),

   #[error("{0}")]
   InvalidValue(String),
}

impl From<dotenvy::Error> for ConfigError {
   fn from(error: dotenvy::Error) -> Self {
      Self::DotEnvyError(error.to_string())
   }
}

pub struct Server {
   pub port: u16,
   pub allow_origins: Vec<String>,
}

pub struct Config {
   pub server: Server,
   pub db_url: String,
   pub jwt_secret: String,
}

impl Config {
   pub fn build() -> Result<Config, ConfigError> {
      load_env_file(".env.sample")?;

      let server = Server {
         port: get_env_value(SERVER_PORT_KEY)?.parse().map_err(|_| {
            ConfigError::InvalidValue(format!("{SERVER_PORT_KEY} env must be number 1024 - 49151"))
         })?,
         allow_origins: get_env_value("ALLOWED_ORIGINS")?
            .split(",")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>(),
      };
      let db_url = get_env_value("DATABASE_URL")?;
      let jwt_secret = get_env_value("JWT_SECRET")?;

      Ok(Config {
         server,
         db_url,
         jwt_secret,
      })
   }
}

fn load_env_file(file: &str) -> Result<(), ConfigError> {
   dotenvy::from_path(Path::new(file))
      .map_err(|e| ConfigError::DotEnvyError(format!("`{file}` {}", e.to_string())))
}

fn get_env_value(key: &str) -> Result<String, ConfigError> {
   dotenvy::var(key).map_err(|e| ConfigError::DotEnvyError(format!("{key} {}", e.to_string())))
}
