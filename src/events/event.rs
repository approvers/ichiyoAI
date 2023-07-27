use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::prelude::Ready;
use serenity::prelude::{Context, EventHandler};

use crate::events::message::chat_ai;

pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.is_private() {
            return;
        }

        if let Ok(true) = msg.mentions_me(&ctx).await {
            chat_ai(&ctx, msg).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!(
            "{} (ID: {}) にログインしました。",
            ready.user.name, ready.user.id
        );
    }
}
