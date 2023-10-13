use crate::client::discord::create_discord_client;
use crate::model::env::{IchiyoAiEnv, ICHIYOAI_ENV};
use dotenvy::dotenv;
use tracing::log::error;

mod client;
mod event;
mod model;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt().compact().init();

    ICHIYOAI_ENV
        .set(envy::from_env::<IchiyoAiEnv>().expect("Failed to load enviroment variables"))
        .unwrap();

    let mut client = create_discord_client(&ICHIYOAI_ENV.get().unwrap().discord_api_token).await?;

    if let Err(why) = client.start().await {
        error!("Failed to starting ichiyoAI: {}", why)
    }

    Ok(())
}
