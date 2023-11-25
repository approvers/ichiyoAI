use crate::model::env::{IchiyoAiEnv, ICHIYOAI_ENV};
use anyhow::Context;
use dotenvy::dotenv;
use event::EvHandler;
use serenity::{model::gateway::GatewayIntents, Client};

mod adapters;
mod event;
mod model;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt().compact().init();

    ICHIYOAI_ENV
        .set(envy::from_env::<IchiyoAiEnv>().expect("Failed to load enviroment variables"))
        .unwrap();

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&ICHIYOAI_ENV.get().unwrap().discord_api_token, intents)
        .event_handler(EvHandler)
        .await
        .context("Failed to create discord client")?;

    if let Err(why) = client.start().await {
        panic!("Failed to starting ichiyoAI: {}", why)
    }

    Ok(())
}
