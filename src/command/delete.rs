use anyhow::Result;

use crate::{Context, database};

#[tracing::instrument]
#[poise::command(slash_command, prefix_command, ephemeral)]
pub async fn delete(ctx: Context<'_>, id: String) -> Result<()> {
    let pool = &ctx.data().pool;

    database::candidate::delete(&id, pool).await?;

    ctx.reply("Successfully deleted").await?;

    Ok(())
}
