use std::sync::RwLock;

pub struct Model {
    comments: RwLock<Vec<String>>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            comments: RwLock::new(vec!["Amazing website!".to_string()]),
        }
    }

    pub fn get_comments(&self) -> std::sync::RwLockReadGuard<'_, Vec<String>> {
        self.comments.read().unwrap()
    }

    pub fn new_comment(&self, text: String) {
        self.comments.write().unwrap().push(text);
    }
}
