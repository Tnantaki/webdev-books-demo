use crate::{
   repos::in_mem::InMemRepos, services::jwt_token::JwtTokenService, startup::config::Config,
};
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
   pub jwt_service: JwtTokenService,
   pub pool: Pool<Postgres>,
   pub in_mem: InMemRepos,
}

impl AppState {
   pub fn new(config: &Config, pool: Pool<Postgres>) -> Self {
      let jwt_service = JwtTokenService::new(&config.jwt_secret);

      Self {
         jwt_service,
         in_mem: InMemRepos::new(),
         pool,
      }
   }
}
