use anyhow::Context;
use dotenvy::dotenv;
use event::EvHandler;
use serenity::{model::gateway::GatewayIntents, Client};

mod adapters;
mod event;
mod model;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let envs = model::env::envs();

    let _guard = sentry::init((
        &*envs.sentry_dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    dotenv().ok();
    tracing_subscriber::fmt().compact().init();

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
