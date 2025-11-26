use anyhow::Result;

use crate::Context;

#[poise::command(slash_command, prefix_command, ephemeral)]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
    ctx.reply("pong").await?;

    Ok(())
}
