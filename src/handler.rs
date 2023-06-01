use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    error::MyError,
    schema::{CreateTaskSchema, CreateUserSchema},
    AppState,
};

pub async fn list_clients_handler(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match app_state.db.fetch_clients().await.map_err(MyError::from) {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn add_client_handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match app_state.db.add_client(&body).await.map_err(MyError::from) {
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

pub async fn get_task_handler(
    Path(skill): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match app_state.db.get_task(&skill).await.map_err(MyError::from) {
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

pub async fn list_freelancers_handler(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match app_state
        .db
        .fetch_freelancers()
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn add_freelancer_handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match app_state
        .db
        .add_freelancer(&body)
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(e) => Err(e.into()),
    }
}
