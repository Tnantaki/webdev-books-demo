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
   models::orders::{OrderItemDetail, OrderModel},
   routes::{JsonResult, app_error::AppError, middleware::auth_cookie},
   startup::app_state::AppState,
};

pub fn router(state: &AppState) -> Router<AppState> {
   Router::new()
      .route("/", get(user_orders))
      .route("/{id}", get(get_order_detail))
      .route("/{id}/pay", put(pay_order))
      .route("/{id}/cancel", put(cancel_order))
      .route("/item/{id}", get(get_order_item_detail))
      .route_layer(middleware::from_fn_with_state(
         state.jwt_service.clone(),
         auth_cookie,
      ))
}

async fn user_orders(
   State(state): State<AppState>,
   Extension(user_id): Extension<Uuid>,
) -> JsonResult<Vec<OrderModel>> {
   let orders = state.postgres.order_repo.get_user_order(user_id).await?;

   Ok((StatusCode::OK, Json(orders)))
}

async fn get_order_detail(
   State(state): State<AppState>,
   Extension(user_id): Extension<Uuid>,
   Path(id): Path<Uuid>,
) -> JsonResult<Vec<OrderItemDetail>> {
   let order_items = state.postgres.order_repo.get_order_detail(user_id, id).await?;

   Ok((StatusCode::OK, Json(order_items)))
}

async fn get_order_item_detail(
   State(state): State<AppState>,
   Extension(user_id): Extension<Uuid>,
   Path(id): Path<Uuid>,
) -> JsonResult<OrderItemDetail> {
   let order_item = state.postgres.order_repo.get_order_item_detail(user_id, id).await?;

   Ok((StatusCode::OK, Json(order_item)))
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
