use anyhow::{Result, anyhow};
use poise::CreateReply;

use crate::{Context, Message};

#[tracing::instrument]
pub async fn is_moderator(ctx: Context<'_>) -> Result<bool> {
    let config = &ctx.data().config;

    let Some(guild) = ctx.guild_id() else {
        return Err(anyhow!("Must be in a guild"));
    };
    let roles = guild.roles(ctx.http()).await?;
    let role_id = roles.iter().find_map(|(id, role)| {
        if role.name == config.moderator_role {
            Some(id)
        } else {
            None
        }
    });
    let Some(role_id) = role_id else {
        return Err(anyhow!("No role with given name"));
    };

    if ctx.author().has_role(ctx.http(), guild, role_id).await? {
        return Ok(true);
    }

    ctx.send(
        CreateReply::default()
            .ephemeral(true)
            .reply(true)
            .content(Message::Unauthorized),
    )
    .await?;

    Ok(false)
}
