use axum::{
   Extension, Json, Router,
   extract::{Path, State},
   http::StatusCode,
   middleware,
   response::IntoResponse,
   routing::{get, put},
};
use serde_json::json;
use uuid::Uuid;

use crate::{
   routes::{
      JsonResult,
      app_error::AppError,
      middleware::{auth_cookie, auth_cookie_admin},
   },
   schemas::order::{Order, OrderDetail},
   startup::app_state::AppState,
};

pub fn router(state: &AppState) -> Router<AppState> {
   Router::new()
      .route("/", get(get_orders_detail))
      .route("/pending", get(get_pending_orders_detail))
      .route("/{id}", get(get_order_by_id))
      .route("/{id}/pay", put(pay_order))
      .route("/{id}/cancel", put(cancel_order))
      .route_layer(middleware::from_fn_with_state(
         state.jwt_service.clone(),
         auth_cookie,
      ))
      .route("/list", get(get_orders))
      .route_layer(middleware::from_fn_with_state(
         state.jwt_service.clone(),
         auth_cookie_admin,
      ))
}

async fn get_orders_detail(
   State(state): State<AppState>,
   Extension(user_id): Extension<Uuid>,
) -> JsonResult<Vec<OrderDetail>> {
   let order_item = state.postgres.order_repo.get_all_orders_detail(user_id).await?;

   Ok((StatusCode::OK, Json(order_item)))
}

async fn get_pending_orders_detail(
   State(state): State<AppState>,
   Extension(user_id): Extension<Uuid>,
) -> JsonResult<Vec<OrderDetail>> {
   let order_item = state.postgres.order_repo.get_pending_orders_detail(user_id).await?;

   Ok((StatusCode::OK, Json(order_item)))
}

async fn get_order_by_id(
   State(state): State<AppState>,
   Extension(user_id): Extension<Uuid>,
   Path(id): Path<Uuid>,
) -> JsonResult<OrderDetail> {
   let order_items = state.postgres.order_repo.get_order_detail_by_id(user_id, id).await?;

   Ok((StatusCode::OK, Json(order_items)))
}

async fn pay_order(
   State(state): State<AppState>,
   Extension(user_id): Extension<Uuid>,
   Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
   state.postgres.order_repo.pay_order(user_id, id).await?;

   Ok((
      StatusCode::OK,
      Json(json!({"message": "pay order succeccfully"})),
   ))
}

async fn cancel_order(
   State(state): State<AppState>,
   Extension(user_id): Extension<Uuid>,
   Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
   state.postgres.order_repo.cancel_order(user_id, id).await?;

   Ok((
      StatusCode::OK,
      Json(json!({"message": "cancel order succeccfully"})),
   ))
}

async fn get_orders(State(state): State<AppState>) -> JsonResult<Vec<Order>> {
   let orders = state.postgres.order_repo.get_all_order().await?;

   Ok((StatusCode::OK, Json(orders)))
}

