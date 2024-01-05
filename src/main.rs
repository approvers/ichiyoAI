use anyhow::Context;
use dotenvy::dotenv;
use event::EvHandler;
use serenity::{model::gateway::GatewayIntents, Client};

mod adapters;
mod event;
mod model;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = sentry::init((
        "https://9b3336d17db273bfd0822c1cd2b01322@sentry.approvers.dev/4",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    dotenv().ok();
    tracing_subscriber::fmt().compact().init();

    let envs = model::env::envs();

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&envs.discord_api_token, intents)
        .event_handler(EvHandler)
        .await
        .context("Failed to create discord client")?;

    if let Err(why) = client.start().await {
        panic!("Failed to starting ichiyoAI: {}", why)
    }

    Ok(())
}
