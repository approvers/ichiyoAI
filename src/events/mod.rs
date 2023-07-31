use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::prelude::{MessageType, Ready};
use serenity::prelude::{Context, EventHandler};
use tracing::info;
use tracing::log::error;

use crate::events::message::chat;

pub mod message;

pub(crate) struct Handler;

const ADMINISTRATOR: u64 = 586824421470109716;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.is_private() {
            return;
        }

        match msg.kind {
            // 通常メッセージ
            MessageType::Regular => {
                if let Ok(false) = msg.mentions_me(&ctx).await {
                    return;
                }

                if let Err(why) = chat(&ctx, &msg).await {
                    let _ = msg
                        .reply_ping(
                            &ctx,
                            &format!(
                                "<@{}> エラーが発生しました。ログを確認してください。\n```{}\n```",
                                ADMINISTRATOR, why
                            ),
                        )
                        .await;
                    error!("{:?}", why)
                }
            }
            _ => {
                return;
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!(
            "{} (ID: {}) にログインしました。",
            ready.user.name, ready.user.id
        );
    }
}
