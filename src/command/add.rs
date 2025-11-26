use anyhow::Result;
use poise::serenity_prelude as serenity;

use crate::{Context, database};

#[tracing::instrument]
#[poise::command(slash_command, prefix_command, ephemeral)]
pub async fn add(ctx: Context<'_>, ids: serenity::Attachment) -> Result<()> {
    let pool = &ctx.data().pool;

    let ids = ids.download().await?;
    let ids = String::from_utf8(ids)?;

    database::candidate::add(ids.trim().lines(), pool).await?;

    ctx.reply("Successfully added").await?;

    Ok(())
}
