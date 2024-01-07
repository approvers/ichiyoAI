extern crate alloc;

use anyhow::Context as _;
use serenity::model::gateway::GatewayIntents;

mod event;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct IchiyoAiEnv {
    pub discord_api_token: String,
    pub openai_api_key: String,
    pub google_ai_api_key: String,
    pub guild_id: u64,
    pub sponsor_role_id: u64,
    pub sentry_dsn: String,
}

pub fn envs() -> &'static IchiyoAiEnv {
    static CACHE: std::sync::OnceLock<IchiyoAiEnv> = std::sync::OnceLock::new();

    CACHE.get_or_init(|| envy::from_env().expect("Failed to load enviroment variables"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let envs = crate::envs();

    // let _guard = sentry::init((
    //     &*envs.sentry_dsn,
    //     sentry::ClientOptions {
    //         release: sentry::release_name!(),
    //         ..Default::default()
    //     },
    // ));

    tracing_subscriber::fmt().compact().init();

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = serenity::Client::builder(&envs.discord_api_token, intents)
        .event_handler(event::EvHandler)
        .await
        .context("Failed to create discord client")?;

    if let Err(why) = client.start().await {
        panic!("Failed to starting ichiyoAI: {}", why)
    }

    Ok(())
}
