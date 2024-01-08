extern crate alloc;

mod event;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct IchiyoAiEnv {
    pub discord_api_token: String,
    pub openai_api_key: String,
    pub google_ai_api_key: String,
    pub guild_id: u64,
    pub sponsor_role_id: u64,
    pub sentry_dsn: Option<String>,
}

pub fn envs() -> &'static IchiyoAiEnv {
    static CACHE: std::sync::OnceLock<IchiyoAiEnv> = std::sync::OnceLock::new();

    CACHE.get_or_init(|| envy::from_env().expect("Failed to load enviroment variables"))
}

#[tokio::main]
async fn main() {
    if let Err(cause) = dotenvy::dotenv() {
        tracing::warn!(%cause, "Failed to load dotenv");
    }

    let envs = crate::envs();

    tracing_subscriber::fmt().compact().init();

    // `sentry::ClientInitGuard` must be stored to keep the transport alive.
    let guard = envs.sentry_dsn.as_ref().map(|dsn| {
        sentry::init((
            dsn.as_str(),
            sentry::ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            },
        ))
    });

    if guard.is_some() {
        tracing::info!("Sentry is initialized");
    } else {
        tracing::warn!("Sentry isn't initialized");
    }

    let intents = serenity::model::gateway::GatewayIntents::GUILD_MESSAGES
        | serenity::model::gateway::GatewayIntents::MESSAGE_CONTENT;

    let result = serenity::Client::builder(&envs.discord_api_token, intents)
        .event_handler(event::EvHandler)
        .await;

    let mut client = match result {
        Ok(ret) => ret,
        Err(cause) => return tracing::error!(%cause, "Failed to create discord client!"),
    };

    if let Err(cause) = client.start().await {
        return tracing::error!(%cause, "Failed to starting ichiyoAI!");
    }
}
