mod models;
mod routes;
mod seed;
mod store;

use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use crate::routes::api;
use crate::routes::web;
use crate::store::AppState;

#[tokio::main]
async fn main() {
    let state = AppState::new(seed::default_records());
    let app = Router::new()
        .route("/", get(web::dashboard_page))
        .route("/records", get(web::records_page))
        .route("/create", get(web::create_page))
        .route("/api/dashboard", get(api::dashboard))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
