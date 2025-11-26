use anyhow::Result;
use sqlx::SqlitePool;

use crate::config::Config;

pub struct State {
    pub pool: SqlitePool,
}

impl State {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;

        Ok(State { pool })
    }
}
