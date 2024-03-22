use sqlx::{postgres::PgPoolOptions, PgPool, Row};

use crate::env;

pub type ChosenDB = PostgresDB;

pub trait DBInterface {
    async fn init() -> Self;
    async fn fetch_comment_contents(&self) -> Vec<String>;
    async fn create_comment(&self, content: &str);
}

pub struct PostgresDB {
    pool: PgPool,
}

impl DBInterface for PostgresDB {
    async fn init() -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(*env::DB_MAX_CONNECTIONS)
            .connect(&env::DB_URL)
            .await
            .unwrap();
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS comments (
                id SERIAL,
                content TEXT
            );"#,
        )
        .execute(&pool)
        .await
        .unwrap();
        Self { pool }
    }

    async fn fetch_comment_contents(&self) -> Vec<String> {
        let row = sqlx::query("SELECT content FROM comments;")
            .fetch_all(&self.pool)
            .await
            .unwrap();
        row.iter().map(|r| r.get::<String, _>("content")).collect()
    }

    async fn create_comment(&self, content: &str) {
        sqlx::query("INSERT INTO comments(content) VALUES ($1);")
            .bind(content)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}
