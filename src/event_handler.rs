use anyhow::{Error, Result};
use poise::serenity_prelude as serenity;

use crate::State;

pub async fn event_handler(
    _: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, State, Error>,
    _: &State,
) -> Result<()> {
    if let serenity::FullEvent::Ready { data_about_bot, .. } = event {
        tracing::info!("Logged in as {}", data_about_bot.user.name);
    }

    Ok(())
}
