use crate::env::load_env;
use client::discord::start_discord_client;
use env::get_env;

mod client;
mod env;
mod event;
mod model;
mod service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_env();
    tracing_subscriber::fmt().compact().init();

    let _guard = if cfg!(feature = "enable_sentry") {
        let _guard = sentry::init((
            "https://f3fd3cb315dd947285acd406f656e0f9@o4505761355988992.ingest.sentry.io/4505761369030656", sentry::ClientOptions {
                release: sentry::release_name!(),
                enable_profiling: true,
                ..Default::default()
            }
        ));
        Some(_guard)
    } else {
        None
    };

    let token = get_env("DISCORD_API_TOKEN");

    start_discord_client(token.as_str()).await?;

    Ok(())
}
