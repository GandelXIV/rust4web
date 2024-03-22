use std::env;

use lazy_static::lazy_static;

fn fetch_var(name: &str, default: Option<&str>) -> String {
    match env::var(name) {
        Ok(v) => v,
        Err(_) => {
            if let Some(d) = default {
                return d.to_string();
            }
            panic!("Missing environment variable {}", name)
        }
    }
}

lazy_static! {
    pub static ref DB_URL: String = fetch_var(
        "DB_URL",
        Some("postgres://postgres:postgres@localhost/postgres")
    );
}
