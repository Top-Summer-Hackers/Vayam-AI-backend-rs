use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
  error::MyError,
  schema::{CreateProviderSchema, CreateTaskSchema},
  AppState,
};

pub async fn list_providers_handler(
  State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match app_state.db.fetch_providers().await.map_err(MyError::from) {
    Ok(res) => Ok(Json(res)),
    Err(e) => Err(e.into()),
  }
}

pub async fn add_provider_handler(
  State(app_state): State<Arc<AppState>>,
  Json(body): Json<CreateProviderSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match app_state
    .db
    .add_provider(&body)
    .await
    .map_err(MyError::from)
  {
    Ok(res) => Ok((StatusCode::CREATED, Json(res))),
    Err(e) => Err(e.into()),
  }
}

pub async fn list_tasks_handler(
  State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match app_state.db.fetch_tasks().await.map_err(MyError::from) {
    Ok(res) => Ok(Json(res)),
    Err(e) => Err(e.into()),
  }
}

pub async fn create_task_handler(
  State(app_state): State<Arc<AppState>>,
  Json(body): Json<CreateTaskSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match app_state.db.create_task(&body).await.map_err(MyError::from) {
    Ok(res) => Ok((StatusCode::CREATED, Json(res))),
    Err(e) => Err(e.into()),
  }
}
