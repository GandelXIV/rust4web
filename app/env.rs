/*
Currently if a variable is not found the regular environment, all dotenv files are parsed and searched.
This is not the fastest, but much easier than keeping state in lazy_static!.
*/

use dotenvy::{self};
use lazy_static::lazy_static;

// Loads variable from .env, then from local environment if not found
fn fetch_var<T: From<String>>(name: &str) -> T {
    println!(
        "Loaded enviroment from {}",
        dotenvy::dotenv_override()
            .unwrap_or("[local]".into())
            .display()
    );

    match dotenvy::var(name) {
        Ok(v) => return v.into(),
        Err(_) => {
            panic!("[PANIC] Missing environment variable {}", name)
        }
    };
}

lazy_static! {
    pub static ref DB_URL: String = fetch_var("DB_URL");
    pub static ref ENV_DEBUG: String = fetch_var("ENV_DEBUG");
}
