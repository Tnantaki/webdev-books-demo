use axum::{Router, response::IntoResponse, routing::get};
use tokio::signal;

use crate::{
   routes::books,
   startup::{ServerError, config::Config},
};

async fn health_check_handler() -> impl IntoResponse {
   "Welcome to book store"
}

pub async fn run(config: Config) -> Result<(), ServerError<'static>> {
   let app = Router::new()
      .route("/health-check", get(health_check_handler))
      .nest("/books", books::routes());

   let listener = tokio::net::TcpListener::bind(("0.0.0.0", config.server.port))
      .await
      .map_err(|e| ServerError::RunServerError(e.to_string()))?;

   println!("Server is running on port: {}", config.server.port);

   axum::serve(listener, app)
      .with_graceful_shutdown(shutdown_signal())
      .await
      .map_err(|e| ServerError::RunServerError(e.to_string()))?;

   Ok(())
}

/// Wait for Ctrl+C or SIGTERM for graceful shutdown
async fn shutdown_signal() {
   let interrupt = async {
      signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
   };

   #[cfg(unix)] // on linux
   let terminate = async {
      let mut term_signal = signal::unix::signal(signal::unix::SignalKind::terminate())
         .expect("Failed to install SIGTERM handler");
      term_signal.recv().await;
   };

   #[cfg(not(unix))] // on Windows
   let terminate = std::future::pending::<()>();

   tokio::select! {
       _ = interrupt => println!("Received SIGINT signal: Shutting down server"),
       _ = terminate => println!("Received SIGTERM signal: Shutting down server"),
   }
}
