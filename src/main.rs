use crate::model::env::{IchiyoAiEnv, ICHIYOAI_ENV};
use anyhow::Context;
use dotenvy::dotenv;
use event::EvHandler;
use serenity::{
    framework::{
        standard::{macros::group, Configuration},
        StandardFramework,
    },
    model::gateway::GatewayIntents,
    Client,
};

use crate::commands::davinci::DAVINCI_COMMAND;

mod adapters;
mod commands;
mod event;
mod model;

#[group]
#[commands(davinci)]
struct Features;

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

    ICHIYOAI_ENV
        .set(envy::from_env::<IchiyoAiEnv>().expect("Failed to load enviroment variables"))
        .unwrap();

    let framework = StandardFramework::new().group(&FEATURES_GROUP);
    framework.configure(Configuration::new().prefix("!"));

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&ICHIYOAI_ENV.get().unwrap().discord_api_token, intents)
        .event_handler(EvHandler)
        .framework(framework)
        .await
        .context("Failed to create discord client")?;

    if let Err(why) = client.start().await {
        panic!("Failed to starting ichiyoAI: {}", why)
    }

    Ok(())
}
