use crate::model::Model;
use axum::{extract::State, response::Html, Form};
use serde::Deserialize;
use std::sync::Arc;

use super::api_read_comments;

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
    api_read_comments::render(State(model)).await
}
