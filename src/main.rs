use anyhow::Context;
use dotenvy::dotenv;
use event::EvHandler;
use serde::{Deserialize, Serialize};
use serenity::{model::gateway::GatewayIntents, Client};
use std::sync::OnceLock;

mod event;

#[derive(Serialize, Deserialize, Debug)]
pub struct IchiyoAiEnv {
    pub discord_api_token: String,
    pub openai_api_key: String,
    pub google_ai_api_key: String,
    pub guild_id: u64,
    pub sponsor_role_id: u64,
    pub sentry_dsn: String,
}

pub fn envs() -> &'static IchiyoAiEnv {
    static CACHE: OnceLock<IchiyoAiEnv> = OnceLock::new();

    CACHE.get_or_init(|| envy::from_env().expect("Failed to load enviroment variables"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let envs = crate::envs();

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
