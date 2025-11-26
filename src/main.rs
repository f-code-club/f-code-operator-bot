pub mod command;
pub mod config;
pub mod database;
pub mod state;

use std::sync::Arc;
use std::time::Duration;

use poise::serenity_prelude::{Client, GatewayIntents};
use poise::{Framework, FrameworkOptions};

pub use crate::config::Config;
pub use crate::state::State;

pub type Context<'a> = poise::Context<'a, State, anyhow::Error>;

pub async fn build_bot() -> anyhow::Result<()> {
    let config = Config::new()?;

    let options = FrameworkOptions {
        commands: vec![command::ping(), command::help(), command::verify()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(config.bot_prefix),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            ..Default::default()
        },
        ..Default::default()
    };

    let framework = Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                State::new(&config.database_url).await
            })
        })
        .options(options)
        .build();

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&config.bot_token, intents)
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .pretty()
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
        .init();

    build_bot().await
}
