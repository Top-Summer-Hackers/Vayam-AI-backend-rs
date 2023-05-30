use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{
  handler::{
    add_freelancer_handler, add_provider_handler, create_task_handler, list_freelancers_handler,
    list_providers_handler, list_tasks_handler,
  },
  AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
  Router::new()
    .route(
      "/api/provider",
      post(add_provider_handler).get(list_providers_handler),
    )
    .route(
      "/api/task",
      post(create_task_handler).get(list_tasks_handler),
    )
    .route(
      "/api/freelancer",
      post(add_freelancer_handler).get(list_freelancers_handler),
    )
    .with_state(app_state)
}
