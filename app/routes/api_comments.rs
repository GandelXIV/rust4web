use crate::{model::Model, views};
use askama::Template;
use axum::{extract::State, response::Html, Form};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct Params {
    newcomment: Option<String>,
}

pub async fn render(State(model): State<Arc<Model>>, Form(data): Form<Params>) -> Html<String> {
    if let Some(newcomment) = data.newcomment {
        if &newcomment != "" {
            model.new_comment(newcomment.clone()).await;
        }
    }

    Html(
        views::CommentShower {
            comments: model.get_comments().await,
        }
        .render()
        .unwrap(),
    )
}
