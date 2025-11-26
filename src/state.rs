use anyhow::Result;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct State {
    pub pool: SqlitePool,
}

impl State {
    #[tracing::instrument]
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        sqlx::migrate!().run(&pool).await.unwrap();

        Ok(State { pool })
    }
}
