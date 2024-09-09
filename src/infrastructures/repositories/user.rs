use crate::infrastructures::models::user::User;
use sqlx::{Error, Pool, Postgres};

pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        UserRepository { pool }
    }
    pub async fn find_all(self) -> Result<Vec<User>, Error> {
        match sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
        {
            Ok(users) => Ok(users),
            Err(e) => Err(e),
        }
    }
}
