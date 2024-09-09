use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub struct Database {
    pub pg_pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(url: String) -> Self {
        let pg_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .expect("Failed to create pool");
        Database { pg_pool }
    }
}
