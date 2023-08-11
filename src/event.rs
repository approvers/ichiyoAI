use crate::client::discord::EvHandler;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::EventHandler;
use tracing::log::info;

#[async_trait]
impl EventHandler for EvHandler {
    async fn ready(&self, ctx: Context, self_bot: Ready) {
        info!(
            "Successfully connected to {username} (ID: {userid})",
            username = self_bot.user.name,
            userid = self_bot.user.id
        )
    }
}
