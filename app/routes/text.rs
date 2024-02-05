use std::sync::Arc;

use crate::{context, model::Model, view};
use axum::{extract::State, response::Html};
use tera::Context;

pub async fn render(State(model): State<Arc<Model>>) -> Html<String> {
    view::render_template(
        "pages/text.html",
        &context! { comments => &(*model.get_comments()) },
    )
}
