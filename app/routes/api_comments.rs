use crate::{context, model::Model, view};
use axum::{extract::State, response::Html, Form};
use serde::Deserialize;
use std::sync::Arc;
use tera::Context;

#[derive(Deserialize)]
pub struct Params {
    newcomment: Option<String>,
}

pub async fn render(State(model): State<Arc<Model>>, Form(data): Form<Params>) -> Html<String> {
    if let Some(newcomment) = data.newcomment {
        model.new_comment(newcomment.clone()).await;
    }
    view::render_template(
        "components/read_comments.html",
        &context! { comments => &*model.get_comments().await },
    )
}
