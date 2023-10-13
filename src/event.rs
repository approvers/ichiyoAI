use crate::model::env::ICHIYOAI_ENV;
use crate::model::EvHandler;
use serenity::async_trait;
use serenity::client::Context;
use serenity::http::{Http, Typing};
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::model::prelude::Activity;
use serenity::prelude::EventHandler;
use std::sync::Arc;
use tracing::log::info;

#[async_trait]
impl EventHandler for EvHandler {
    async fn ready(&self, ctx: Context, self_bot: Ready) {
        info!("Starting...");

        let version = &ICHIYOAI_ENV.get().unwrap().cargo_pkg_version;
        ctx.set_activity(Activity::playing(&format!("v{}", version)))
            .await;

        info!("Running ichiyoAI v{}", version);
        info!(
            "Connected!: {name}(Id:{id})",
            name = self_bot.user.name,
            id = self_bot.user.id
        )
    }
}

fn start_typing(http: Arc<Http>, target_channel_id: ChannelId) -> Typing {
    Typing::start(http, u64::from(target_channel_id)).expect("タイピングを開始できませんでした.")
}
