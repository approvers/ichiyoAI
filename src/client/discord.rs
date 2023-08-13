use anyhow::Context;
use serenity::{prelude::GatewayIntents, Client};

pub struct EvHandler;

pub async fn start_discord_client(token: &str) -> anyhow::Result<()> {
    // メッセージ内容の取得とギルドメッセージの取得を有効化
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(EvHandler)
        .await
        .context("Failed to create a client")?;

    client.start().await.context("Failed to start a client")
}
