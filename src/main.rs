pub mod command;
pub mod config;
pub mod state;

pub use crate::config::Config;
pub use crate::state::State;

pub type Context<'a> = poise::Context<'a, State, anyhow::Error>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .pretty()
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
        .init();
}
