use anyhow::Result;
use poise::builtins::HelpConfiguration;

use crate::Context;

#[tracing::instrument]
#[poise::command(slash_command, prefix_command, ephemeral)]
pub async fn help(ctx: Context<'_>, command: Option<String>) -> Result<()> {
    let configuration = HelpConfiguration::default();
    poise::builtins::help(ctx, command.as_deref(), configuration).await?;

    Ok(())
}
