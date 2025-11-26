use anyhow::Result;
use sqlx::SqlitePool;

use crate::config::Config;

pub struct State {
    pub pool: SqlitePool,
}

impl State {
    pub async fn new(config: &Config) -> Result<Self> {
        let pool = SqlitePool::connect(&config.database_url).await?;

        Ok(State { pool })
    }
}
