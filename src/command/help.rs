use anyhow::Result;
use poise::builtins::PrettyHelpConfiguration;

use crate::Context;

#[tracing::instrument]
#[poise::command(slash_command, prefix_command, ephemeral)]
pub async fn help(ctx: Context<'_>, command: Option<String>) -> Result<()> {
    let configuration = PrettyHelpConfiguration::default();
    poise::builtins::pretty_help(ctx, command.as_deref(), configuration).await?;

    Ok(())
}
