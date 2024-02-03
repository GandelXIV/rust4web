use std::sync::RwLock;

pub struct Model {
    pub score: RwLock<usize>,
    comments: RwLock<Vec<String>>,
}

const START_SCORE: usize = 0;

impl Model {
    pub fn new() -> Self {
        Self {
            score: RwLock::new(START_SCORE),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scoring() {
        let model = Model::new();
        assert_eq!(model.get_score(), START_SCORE);
        model.increment_score();
        assert_eq!(model.get_score(), START_SCORE + 1);
    }
}
