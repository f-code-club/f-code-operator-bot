use anyhow::Result;

use crate::{Context, Message, database, util};

/// Delete a candidate by ID from the candidates database.
#[tracing::instrument]
#[poise::command(
    slash_command,
    prefix_command,
    ephemeral,
    required_permissions = "MANAGE_MESSAGES | MANAGE_THREADS"
)]
pub async fn delete(ctx: Context<'_>, id: String) -> Result<()> {
    let pool = &ctx.data().pool;

    if !util::is_valid_id(&id) {
        ctx.reply(Message::InvalidId).await?;
        return Ok(());
    }

    database::candidate::delete(&id, pool).await?;

    ctx.reply(Message::CandidateDeleted(&id)).await?;

    Ok(())
}
