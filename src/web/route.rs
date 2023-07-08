use crate::handler::{
  add_milestones_handler, get_proposal_handler, list_milestone_handler, submit_milestone_handler,
};
use crate::web::mw_auth::{mw_require_auth};
use crate::{handler::{
  add_client_handler, add_freelancer_handler, add_review_handler, api_login_handler,
  approve_proposal_handler, create_task_handler, get_task_handler, list_clients_handler,
  list_deals_handler, list_freelancers_handler, list_proposal_handler, list_tasks_handler,
  submit_proposal_handler, update_deal_handler
}, AppState, web};
use axum::response::Response;
use axum::{
  middleware,
  routing::{get, patch, post},
  Router,
};
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;

pub fn create_router(app_state: Arc<AppState>) -> Router {
  Router::new()
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
      get(get_proposal_handler).patch(approve_proposal_handler),
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
    .route("/api/freelancer", get(list_freelancers_handler), )
    .route("/api/review", post(add_review_handler))
    .route("/api/client", get(list_clients_handler)) //provider, employee
    .layer(middleware::map_response(main_response_mapper))
    .layer(middleware::from_fn(mw_require_auth))
    .route("/api/client", post(add_client_handler))
    .route("/api/freelancer", post(add_freelancer_handler))
    .route("/api/login", post(api_login_handler))
    .layer(CookieManagerLayer::new())
    .with_state(app_state)
}

async fn main_response_mapper(res: Response) -> Response {
  println!("--> {:<12} - main_response_mapper middleware", "INFO");
  res
}
