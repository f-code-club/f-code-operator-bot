use anyhow::Result;
use poise::CreateReply;

use crate::Context;

#[poise::command(slash_command, ephemeral)]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
    ctx.send(CreateReply::default().ephemeral(true).content("pong"))
        .await?;

    Ok(())
}
