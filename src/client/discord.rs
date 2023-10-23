use crate::model::EvHandler;
use anyhow::Context;
use serenity::{prelude::GatewayIntents, Client};

pub async fn create_discord_client(token: &str) -> anyhow::Result<Client> {
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let client = Client::builder(token, intents)
        .event_handler(EvHandler)
        .await
        .context("Failed to create discord client")?;

    Ok(client)
}
