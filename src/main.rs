use std::env;

use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::prelude::{Client, GatewayIntents};

use crate::commands::DIRECT_COMMAND;
use crate::events::Handler;

mod api;
mod commands;
mod events;

#[group]
#[commands(direct)]
struct Conversation;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_API_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;

    let framework = StandardFramework::new()
        .configure(|f| f.prefix("!"))
        .group(&CONVERSATION_GROUP);

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("クライアントの作成に失敗しました。");

    if let Err(why) = client.start().await {
        println!("クライアントの起動に失敗しました: {:?}", why)
    }
}
