use std::sync::Mutex;

use self::db::{ChosenDB, DBInterface};
mod db;

pub struct Model {
    database: ChosenDB,
    comment_cache: Mutex<Vec<String>>,
}

impl Model {
    pub async fn new() -> Self {
        let database = db::ChosenDB::init().await;
        let comment_cache = Mutex::new(database.fetch_comment_contents().await);
        Self {
            database,
            comment_cache
        }
    }

    pub async fn get_comments(&self) -> Vec<String> {
        self.comment_cache.lock().unwrap().clone()
        //self.database.fetch_comment_contents().await
    }

    pub async fn new_comment(&self, text: String) {
        self.database.create_comment(&text).await;
        self.comment_cache.lock().unwrap().push(text);
    }
}
