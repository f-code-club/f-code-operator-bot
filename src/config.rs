use std::sync::LazyLock;

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

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    ::config::Config::builder()
        .add_source(
            ::config::Environment::default()
                .try_parsing(true)
                .separator("__"),
        )
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
});
