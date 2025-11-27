use anyhow::Result;
use sqlx::SqlitePool;

use crate::Config;

#[derive(Debug)]
pub struct State {
    pub pool: SqlitePool,
    pub config: Config,
}

impl State {
    #[tracing::instrument]
    pub async fn new(config: Config) -> Result<Self> {
        let pool = SqlitePool::connect(&config.database_url).await?;
        sqlx::migrate!().run(&pool).await.unwrap();

        Ok(State { pool, config })
    }
}
