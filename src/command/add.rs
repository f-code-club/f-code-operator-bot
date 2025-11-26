use anyhow::Result;
use poise::serenity_prelude as serenity;

use crate::{Context, Message, database, util};

#[tracing::instrument]
#[poise::command(slash_command, prefix_command, ephemeral)]
pub async fn add(ctx: Context<'_>, id: serenity::Attachment) -> Result<()> {
    let pool = &ctx.data().pool;

    let id = id.download().await?;
    let id = String::from_utf8(id)?;
    for id in id.lines() {
        if util::is_valid_id(id) {
            continue;
        }

        ctx.reply(Message::InvalidId).await?;
        return Ok(());
    }

    database::candidate::add(id.trim().lines(), pool).await?;

    ctx.reply(Message::CandidateAdded).await?;

    Ok(())
}
