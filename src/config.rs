use anyhow::Result;
use serde::Deserialize;

fn default_database_url() -> String {
    "data.sql".to_string()
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_database_url")]
    pub database_url: String,

    pub discord_token: String,
}

impl Config {
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
