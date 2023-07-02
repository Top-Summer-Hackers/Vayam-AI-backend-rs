use crate::handler::{
  add_milestones_handler, get_proposal_handler, list_milestone_handler, submit_milestone_handler,
};
use crate::web::mw_auth::mw_require_auth;
use crate::{
  handler::{
    add_client_handler, add_freelancer_handler, add_review_handler, api_login_handler,
    aprove_proposal_handler, create_task_handler, get_task_handler, list_clients_handler,
    list_deals_handler, list_freelancers_handler, list_proposal_handler, list_tasks_handler,
    submit_proposal_handler, update_deal_handler,
  },
  AppState,
};
use axum::{
  middleware,
  routing::{get, patch, post},
  Router,
};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
  Router::new()
    .route("/api/login", post(api_login_handler))
    //.route_layer(middleware::from_fn(mw_require_auth))
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
    .route(
      "/api/proposal/:proposal_id",
      get(get_proposal_handler).patch(aprove_proposal_handler),
    )
    .route(
      "/api/milestone",
      post(add_milestones_handler).get(list_milestone_handler),
    )
    .route(
      "/api/milestone/:proposal_id/:milestone_id/:link",
      patch(submit_milestone_handler),
    )
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
