use std::sync::Arc;

use axum::{
  extract::{Path, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use tower_cookies::Cookies;

use crate::{
  error::MyError,
  schema::{
    CreateMilestoneSchema, CreateProposalSchema, CreateReviewSchema, CreateTaskSchema,
    CreateUserSchema, LoginUserSchema,
  },
  AppState,
};
use crate::response::SingleUserResponse;
use crate::web::token::generate_auth_cookie;

fn when_user_added(
    result: Result<SingleUserResponse, MyError>,
    cookies: Cookies
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match result {
    Ok(res) => {
      cookies.add(generate_auth_cookie(res.data.user.id.clone(), None));
      Ok((StatusCode::CREATED, Json(res)))
    }
    Err(e) => Err(e.into()),
  }
}

pub async fn api_login_handler(
  cookies: Cookies,
  State(app_state): State<Arc<AppState>>,
  Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match app_state
    .db
    .api_login(cookies, &body)
    .await
    .map_err(MyError::from)
  {
    Ok(res) => Ok((StatusCode::OK, Json(res))),
    Err(e) => Err(e.into()),
  }
}

pub async fn list_clients_handler(
  State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match app_state.db.fetch_clients().await.map_err(MyError::from) {
    Ok(res) => Ok(Json(res)),
    Err(e) => Err(e.into()),
  }
}

pub async fn add_client_handler(
  cookies: Cookies,
  State(app_state): State<Arc<AppState>>,
  Json(body): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  when_user_added(app_state.db.add_client(&body).await.map_err(MyError::from), cookies)
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

pub async fn get_proposal_handler(
  Path(proposal_id): Path<String>,
  State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match app_state
    .db
    .get_proposal(&proposal_id)
    .await
    .map_err(MyError::from)
  {
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
  cookies: Cookies,
  State(app_state): State<Arc<AppState>>,
  Json(body): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
   when_user_added(app_state.db.add_freelancer(&body).await.map_err(MyError::from), cookies)
}

pub async fn add_review_handler(
  State(app_state): State<Arc<AppState>>,
  Json(body): Json<CreateReviewSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match app_state.db.add_review(&body).await.map_err(MyError::from) {
    Ok(res) => Ok((StatusCode::CREATED, Json(res))),
    Err(e) => Err(e.into()),
  }
}

pub async fn list_proposal_handler(
  State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match app_state.db.fetch_proposals().await.map_err(MyError::from) {
    Ok(res) => Ok(Json(res)),
    Err(e) => Err(e.into()),
  }
}

pub async fn list_milestone_handler(
  State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match app_state.db.fetch_milestones().await.map_err(MyError::from) {
    Ok(res) => Ok(Json(res)),
    Err(e) => Err(e.into()),
  }
}
pub async fn submit_proposal_handler(
  State(app_state): State<Arc<AppState>>,
  Json(body): Json<CreateProposalSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match app_state
    .db
    .submit_proposal(&body)
    .await
    .map_err(MyError::from)
  {
    Ok(res) => Ok((StatusCode::CREATED, Json(res))),
    Err(e) => Err(e.into()),
  }
}

pub async fn add_milestones_handler(
  State(app_state): State<Arc<AppState>>,
  Json(body): Json<Vec<CreateMilestoneSchema>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match app_state
    .db
    .add_milestones(&body)
    .await
    .map_err(MyError::from)
  {
    Ok(res) => Ok((StatusCode::CREATED, Json(res))),
    Err(e) => Err(e.into()),
  }
}
pub async fn approve_proposal_handler(
  Path(proposal_id): Path<String>,
  State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match app_state
    .db
    .approve_proposal(&proposal_id)
    .await
    .map_err(MyError::from)
  {
    Ok(res) => Ok((StatusCode::CREATED, Json(res))),
    Err(e) => Err(e.into()),
  }
}

pub async fn list_deals_handler(
  State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match app_state.db.fetch_deals().await.map_err(MyError::from) {
    Ok(res) => Ok(Json(res)),
    Err(e) => Err(e.into()),
  }
}

pub async fn update_deal_handler(
  Path((deal_id, proposal_id)): Path<(String, String)>,
  State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match app_state
    .db
    .update_deal(&deal_id, &proposal_id)
    .await
    .map_err(MyError::from)
  {
    Ok(res) => Ok((StatusCode::CREATED, Json(res))),
    Err(e) => Err(e.into()),
  }
}

pub async fn submit_milestone_handler(
  Path((proposal_id, milestone_id, link)): Path<(String, String, String)>,
  State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match app_state
    .db
    .submit_milestone(&proposal_id, &milestone_id, &link)
    .await
    .map_err(MyError::from)
  {
    Ok(res) => Ok((StatusCode::CREATED, Json(res))),
    Err(e) => Err(e.into()),
  }
}
