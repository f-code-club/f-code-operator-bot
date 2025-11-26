use anyhow::{Result, anyhow};
use chrono::Local;
use poise::serenity_prelude::EditRole;

use crate::{Context, Message, database, util};

const ROLE: &str = "Round 1: Challenger";

#[tracing::instrument]
#[poise::command(slash_command, prefix_command, ephemeral)]
pub async fn verify(ctx: Context<'_>, id: String) -> Result<()> {
    let pool = &ctx.data().pool;

    if !util::is_valid_id(&id) {
        ctx.reply(Message::InvalidId).await?;
        return Ok(());
    }

    let Some(candidate) = database::candidate::get(&id, pool).await? else {
        ctx.reply(Message::NotRegistered).await?;

        return Ok(());
    };
    if let Some(verification_time) = candidate.verification_time {
        ctx.reply(Message::Verified(Some(verification_time)))
            .await?;

        return Ok(());
    }

    tracing::info!(author = ?ctx.author_member().await);

    let Some(member) = ctx.author_member().await else {
        return Err(anyhow!("Message contain no author"));
    };

    let possible_names = [
        member.nick.as_ref(),
        member.user.global_name.as_ref(),
        Some(&member.user.name),
    ];
    let name = possible_names.iter().flatten().next().unwrap();
    if !name.contains(&id) {
        ctx.reply(Message::InvalidName).await?;

        return Ok(());
    }

    database::candidate::verify(&id, &ctx.data().pool).await?;

    let Some(guild) = ctx.guild_id() else {
        return Err(anyhow!("Must be in a guild"));
    };
    let role = guild
        .create_role(ctx.http(), EditRole::new().name(ROLE))
        .await?;
    member.add_role(ctx.http(), role.id).await?;

    ctx.reply(Message::Verified(None)).await?;

    Ok(())
}
