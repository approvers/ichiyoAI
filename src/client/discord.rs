use anyhow::Context;
use serenity::{prelude::GatewayIntents, Client};

pub struct EvHandler;

pub async fn start_discord_client(token: &str) -> anyhow::Result<()> {
    // メッセージ内容の取得とギルドメッセージの取得を有効化
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(EvHandler)
        .await
        .context("クライアントの作成に失敗しました.")?;

    client
        .start()
        .await
        .context("クライアントの起動に失敗しました.")
}
