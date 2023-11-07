use crate::client::discord::create_discord_client;
use crate::model::env::{IchiyoAiEnv, ICHIYOAI_ENV};
use dotenvy::dotenv;
use model::env::LogEnvironment;
use tracing::log::error;
use tracing::Level;

mod adapters;
mod client;
mod event;
mod model;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    ICHIYOAI_ENV
        .set(envy::from_env::<IchiyoAiEnv>().expect("Failed to load enviroment variables"))
        .unwrap();

    // TODO: 改善の余地あり
    match ICHIYOAI_ENV.get().unwrap().log_environment {
        LogEnvironment::Production => {
            tracing_subscriber::fmt()
                .with_max_level(Level::INFO)
                .compact()
                .init();
        }
        LogEnvironment::Development => {
            tracing_subscriber::fmt()
                .with_max_level(Level::DEBUG)
                .compact()
                .init();
        }
    }

    let mut client = create_discord_client(&ICHIYOAI_ENV.get().unwrap().discord_api_token).await?;

    if let Err(why) = client.start().await {
        error!("Failed to starting ichiyoAI: {}", why)
    }

    Ok(())
}
