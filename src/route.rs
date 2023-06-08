use crate::{
  handler::{
    add_client_handler, add_freelancer_handler, add_review_handler, aprove_proposal_handler,
    create_task_handler, get_task_handler, list_clients_handler, list_deals_handler,
    list_freelancers_handler, list_proposal_handler, list_tasks_handler, submit_proposal_handler,
    update_deal_handler,
  },
  AppState,
};
use axum::{
  routing::{get, patch, post},
  Router,
};
use std::sync::Arc;

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
    .route("/api/task/:skill", get(get_task_handler))
    .route(
      "/api/proposal",
      post(submit_proposal_handler).get(list_proposal_handler),
    )
    .route("/api/proposal/:proposal_id", patch(aprove_proposal_handler))
    .route("/api/deal", get(list_deals_handler))
    .route(
      "/api/deal/:deal_id/:transacion_id",
      patch(update_deal_handler),
    )
    .route(
      "/api/freelancer",
      post(add_freelancer_handler).get(list_freelancers_handler),
    )
    .route("/api/review", post(add_review_handler))
    .with_state(app_state)
}
