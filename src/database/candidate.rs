use chrono::{Local, NaiveDateTime};
use sqlx::{Result, SqliteExecutor, SqlitePool};

#[tracing::instrument]
pub async fn create(ids: &[String], pool: &SqlitePool) -> Result<()> {
    let mut transaction = pool.begin().await?;

    for id in ids {
        sqlx::query!("INSERT INTO candidates (id) VALUES($1)", id)
            .execute(&mut *transaction)
            .await?;
    }

    transaction.commit().await
}

pub struct Candidate {
    pub id: String,
    pub verification_time: Option<NaiveDateTime>,
}

pub async fn get(
    id: &str,
    executor: impl SqliteExecutor<'_>,
) -> Result<Option<Candidate>> {
    sqlx::query_as!(
        Candidate,
        r#"
            SELECT id, verification_time as "verification_time: NaiveDateTime"
            FROM candidates
            WHERE id = $1
        "#,
        id
    )
    .fetch_optional(executor)
    .await
}

pub async fn verify(id: &str, executor: impl SqliteExecutor<'_>) -> Result<()> {
    let now = Local::now().naive_local();

    sqlx::query!(
        r#"
            UPDATE candidates
            SET verification_time = $2
            WHERE id = $1
        "#,
        id,
        now
    )
    .execute(executor)
    .await?;

    Ok(())
}
