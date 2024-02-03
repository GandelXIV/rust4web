use crate::{
    context,
    model::{self, Model},
    views,
};
use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tera::Context;

pub fn build_routes() -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/comments", post(comments))
        .with_state(Arc::new(model::Model::new()))
}

// Here go routes

async fn hello(State(model): State<Arc<Model>>) -> Html<String> {
    views::render_template(
        "pages/greet.html",
        &context! { name => "World", score => &model.get_score(), comments => &*model.get_comments()},
    )
}

#[derive(Deserialize)]
struct CommentData {
    newcomment: Option<String>,
}

async fn comments(State(model): State<Arc<Model>>, Form(data): Form<CommentData>) -> Html<String> {
    if let Some(newcomment) = data.newcomment {
        model.new_comment(newcomment.clone());
    }
    views::render_template(
        "components/read_comments.html",
        &context! { comments => &*model.get_comments() },
    )
}
