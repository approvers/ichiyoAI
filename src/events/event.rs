use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::prelude::Ready;
use serenity::prelude::{Context, EventHandler};

use crate::events::message::chat_ai;

pub(crate) struct Handler;

const ADMINISTRATOR: u64 = 586824421470109716;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.is_private() {
            return;
        }

        if let Ok(true) = msg.mentions_me(&ctx).await {
            // TODO: GPT-4が使えるようになったら解放する
            if let Err(why) = chat_ai(&ctx, &msg, false).await {
                let _ = msg
                    .channel_id
                    .send_message(&ctx, |m| {
                        m.content(&format!(
                            "<@{}> エラーが発生しました。ログを確認してください。",
                            ADMINISTRATOR
                        ))
                    })
                    .await;
                eprintln!("{:?}", why)
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!(
            "{} (ID: {}) にログインしました。",
            ready.user.name, ready.user.id
        );
    }
}
