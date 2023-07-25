use std::env;

use serenity::prelude::{Client, GatewayIntents};

use events::event::Handler;

mod api;
mod events;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_API_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("クライアントの作成に失敗しました。");

    if let Err(why) = client.start().await {
        println!("クライアントの起動に失敗しました: {:?}", why)
    }
}
