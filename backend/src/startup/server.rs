use axum::{Router, response::IntoResponse, routing::get};
use chrono::Utc;
use console::style;
use sqlx::{Pool, Postgres};
use tokio::signal;
use tokio_cron_scheduler::{Job, JobScheduler};
use tower_cookies::CookieManagerLayer;

use crate::{
   repos::postgres::images::ImageRepo, routes::{auth, books, cart_items, images, users}, startup::{app_state::AppState, config::Config}, ServerError
};

async fn health_check_handler() -> impl IntoResponse {
   "Welcome to book store"
}

pub async fn run(config: Config, pool: Pool<Postgres>) -> Result<(), ServerError<'static>> {
   // Share app state in multiple route, use arc
   let app_state = AppState::new(&config, pool.clone());

   // This spawns a new asynchronous scheduler in a background task ⬇️
   tokio::spawn(async move {
      println!("Starting cleanup scheduler in background...");
      if let Err(e) = run_cleanup_scheduler(pool).await {
         eprintln!("{} {}", style("Scheduler error:").red().bold(), e);
      }
   });

   let app = Router::new()
      .route("/health", get(health_check_handler))
      .nest("/auth", auth::router())
      .nest("/users", users::router(&app_state))
      .nest("/books", books::router(&app_state))
      .nest("/images", images::router(&app_state))
      .nest("/cart", cart_items::router(&app_state))
      .with_state(app_state)
      .layer(CookieManagerLayer::new());

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

async fn run_cleanup_scheduler(pool: Pool<Postgres>) -> Result<(), Box<dyn std::error::Error>> {
   let sched = JobScheduler::new().await?;

   // Add basic cron job
   sched
      .add(
         // run every hours
         Job::new_async("0 0 * * * *", move |_uuid, _l| {
            let image_repo = ImageRepo::new(pool.clone());

            Box::pin(async move {
               let now = format!("[{}]", Utc::now());
               println!("{} Running scheduled cleanup...", style(now).green());
               // clean up the image that orphan more than 24 hours
               if let Err(e) = image_repo.cleanup_orphan_images(24).await {
                  eprintln!("Cleanup failed: {}", e);
               }
            })
         })?,
      )
      .await?;

   // Start the scheduler
   sched.start().await?;

   tokio::signal::ctrl_c().await?;
   Ok(())
}
