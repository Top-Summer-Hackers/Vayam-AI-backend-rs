use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{
  handler::{
    add_client_handler, add_freelancer_handler, create_task_handler, list_clients_handler,
    list_freelancers_handler, list_tasks_handler,
  },
  AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
  Router::new()
    .route(
      "/api/client", //provider, employee
      post(add_client_handler).get(list_clients_handler),
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
