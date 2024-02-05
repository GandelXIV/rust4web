use axum::response::Html;
use lazy_static::lazy_static;
use tera::Context;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = Tera::new("templates/**/*.html").unwrap();
}

#[macro_export]
macro_rules! context {
    (
        $(
            $key:ident $(=> $value:expr)? $(,)*
        )*
    ) => {
        {
            let mut context = Context::new();
            $(
                context.insert(stringify!($key), $($value)?);
            )*
            context
        }
    };
}

pub fn render_template(name: &str, ctx: &Context) -> Html<String> {
    Html(TEMPLATES.render(name, ctx).unwrap())
}
