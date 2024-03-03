use crate::{model::Model, views};
use askama::Template;
use axum::{extract::State, response::Html};
use std::sync::Arc;

pub async fn render(State(model): State<Arc<Model>>) -> Html<String> {
    Html(
        views::CommentShower {
            comments: model.get_comments().await,
        }
        .render()
        .unwrap(),
    )
}
