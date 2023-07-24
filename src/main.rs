use std::env;
use serenity::Client;
use serenity::prelude::GatewayIntents;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_API_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILDS;

    let mut client = Client::builder(token, intents)
        .await
        .expect("クライアントの作成に失敗しました。");

    if let Err(why) = client.start().await {
        println!("クライアントの起動に失敗しました: {:?}", why);
    }
}
