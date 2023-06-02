mod db;
mod error;
mod handler;
mod model;
mod response;
mod route;
mod schema;
mod utils;

use std::{net::SocketAddr, sync::Arc};

use axum::http::{
  header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
  HeaderValue, Method,
};
use db::DB;
use dotenv::dotenv;
use error::MyError;
use route::create_router;
use tower_http::cors::CorsLayer;

pub struct AppState {
  db: DB,
}

#[tokio::main]
async fn main() -> Result<(), MyError> {
  dotenv().ok();

  let db = DB::init().await?;

  // let cors = CorsLayer::new()
  //   .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
  //   .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
  //   .allow_credentials(true)
  //   .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

  let app = create_router(Arc::new(AppState { db: db.clone() })); //.layer(cors);

  let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
  println!("✅ Server listening on {addr}\n");
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();

  Ok(())
}
