use crate::env::load_env;
use client::discord::start_discord_client;
use env::get_env;

mod api;
mod client;
mod env;
mod event;
mod service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_env();
    tracing_subscriber::fmt().compact().init();

    let token = get_env("DISCORD_API_TOKEN");

    start_discord_client(token.as_str()).await?;

    Ok(())
}
