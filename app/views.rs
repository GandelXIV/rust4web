use askama::Template; // bring trait in scope

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct Home<'a> {
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "pages/text.html")]
pub struct Text {
    pub comments: Vec<String>,
}

#[derive(Template)]
#[template(path = "components/comment_shower.html")]
pub struct CommentShower {
    pub comments: Vec<String>,
}
