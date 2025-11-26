use anyhow::{Result, anyhow};

use crate::{Context, database};

#[tracing::instrument]
#[poise::command(slash_command, prefix_command, ephemeral)]
pub async fn verify(ctx: Context<'_>, id: String) -> Result<()> {
    let pool = &ctx.data().pool;

    let Some(candidate) = database::candidate::get(&id, pool).await? else {
        ctx.reply("You have not registered").await?;

        return Ok(());
    };
    if let Some(verification_time) = candidate.verification_time {
        ctx.reply(format!("You have verified at {}", verification_time))
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
        ctx.reply("Name should contain student id").await?;

        return Ok(());
    }

    database::candidate::verify(&id, &ctx.data().pool).await?;

    ctx.reply("Successfully verify").await?;

    Ok(())
}
