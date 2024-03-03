use crate::model;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;

// routefiles
mod api_comments;
mod home;
mod text;

pub async fn build_routes() -> Router {
    let model = model::Model::new().await;
    Router::new()
        .route("/", get(home::render))
        .route("/text", get(text::render))
        .route("/api/comments", post(api_comments::render))
        .with_state(Arc::new(model))
}
