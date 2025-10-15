use axum::{
  routing::{get, post},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize)]
struct Hello {
  message: String,
}

#[derive(Deserialize)]
struct CreateUser {
  name: String,
}

async fn hello_world() -> Json<Hello> {
  Json(Hello {
    message: "Hello, Axum!".to_string(),
  })
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<Hello> {
  Json(Hello {
    message: format!("User {} created", payload.name),
  })
}

#[tokio::main]
async fn main() {
  let app = Router::new()
    .route("/", get(hello_world))
    .route("/users", post(create_user));

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  println!("Server running on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}