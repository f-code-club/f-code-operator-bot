use anyhow::Result;
use serde::Deserialize;

fn default_database_url() -> String {
    "sqlite:data.db?mode=rwc".to_string()
}

fn default_bot_prefix() -> String {
    "!".to_string()
}

fn default_candidate_role() -> String {
    "Round 1: Challenger".to_string()
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_database_url")]
    pub database_url: String,

    #[serde(default = "default_bot_prefix")]
    pub bot_prefix: String,

    pub bot_token: String,

    #[serde(default = "default_candidate_role")]
    pub candidate_role: String,
}

impl Config {
    #[tracing::instrument]
    pub fn new() -> Result<Self> {
        let config = ::config::Config::builder()
            .add_source(
                ::config::Environment::default()
                    .try_parsing(true)
                    .separator("__"),
            )
            .build()?
            .try_deserialize()?;

        tracing::info!(?config);

        Ok(config)
    }
}
