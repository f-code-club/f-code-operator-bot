use anyhow::Result;
use serde::Deserialize;

fn default_database_url() -> String {
    "sqlite:data.db".to_string()
}

fn default_bot_prefix() -> String {
    "!".to_string()
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_database_url")]
    pub database_url: String,

    #[serde(default = "default_bot_prefix")]
    pub bot_prefix: String,

    pub bot_token: String,
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

        Ok(config)
    }
}
