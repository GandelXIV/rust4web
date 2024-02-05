use crate::{context, view};
use axum::response::Html;
use tera::Context;

pub async fn render() -> Html<String> {
    view::render_template("pages/home.html", &context! { name => "World" })
}
