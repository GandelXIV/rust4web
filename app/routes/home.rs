use crate::views;
use askama::Template;
use axum::response::Html;

pub async fn render() -> Html<String> {
    let page = views::Home { name: "World" };
    Html(page.render().unwrap())
}
