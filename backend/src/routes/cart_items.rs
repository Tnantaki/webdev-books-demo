use axum::{
   Extension, Json, Router,
   extract::{Path, State},
   http::StatusCode,
   middleware,
   routing::{delete, get, post, put},
};
use uuid::Uuid;
use validator::Validate;

use crate::{
   models::{cart_items::CartItemModel, orders::OrderModel},
   routes::{JsonResult, app_error::AppError, middleware::auth_cookie},
   schemas::cart::{AddCartItem, EditCartItem},
   startup::app_state::AppState,
};

pub fn router(state: &AppState) -> Router<AppState> {
   Router::new()
      .route("/", get(get_cart_items))
      .route("/item", post(add_to_cart))
      .route("/item/{id}", put(edit_cart_item))
      .route("/item/{id}", delete(remove_from_cart))
      .route("/checkout", post(checkout))
      .route_layer(middleware::from_fn_with_state(
         state.jwt_service.clone(),
         auth_cookie,
      ))
}

async fn add_to_cart(
   State(state): State<AppState>,
   Extension(user_id): Extension<Uuid>,
   Json(payload): Json<AddCartItem>,
) -> JsonResult<CartItemModel> {
   // 1. validate input
   payload.validate()?;

   // 2. add to database
   let cart_item = state.postgres.cart_item_repo.add_to_cart(user_id, payload).await?;
   // 3. response
   Ok((StatusCode::CREATED, Json(cart_item)))
}

async fn get_cart_items(
   State(state): State<AppState>,
   Extension(user_id): Extension<Uuid>,
) -> JsonResult<Vec<CartItemModel>> {
   let cart = state.postgres.cart_item_repo.get_cart_items(user_id).await?;

   Ok((StatusCode::CREATED, Json(cart)))
}

async fn edit_cart_item(
   State(state): State<AppState>,
   Extension(user_id): Extension<Uuid>,
   Path(id): Path<Uuid>,
   Json(payload): Json<EditCartItem>,
) -> JsonResult<CartItemModel> {
   // Validate
   payload.validate()?;

   // Update data in database
   let cart_item = state.postgres.cart_item_repo.edit_cart_item(user_id, id, payload).await?;

   Ok((StatusCode::OK, Json(cart_item)))
}

async fn remove_from_cart(
   State(state): State<AppState>,
   Extension(user_id): Extension<Uuid>,
   Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
   state.postgres.cart_item_repo.remove_from_cart(user_id, id).await?;

   Ok(StatusCode::NO_CONTENT)
}

async fn checkout(
   State(state): State<AppState>,
   Extension(user_id): Extension<Uuid>,
) -> JsonResult<OrderModel> {
   let order = state.postgres.cart_item_repo.create_order_from_cart(user_id).await?;

   Ok((StatusCode::CREATED, Json(order)))
}
