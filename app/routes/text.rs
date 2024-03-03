use std::sync::Arc;

use crate::{model::Model, views};
use askama::Template;
use axum::{extract::State, response::Html};

pub async fn render(State(model): State<Arc<Model>>) -> Html<String> {
    Html(
        views::Text {
            comments: model.get_comments().await,
        }
        .render()
        .unwrap(),
    )
}
