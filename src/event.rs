use std::sync::Arc;

use serenity::async_trait;
use serenity::client::Context;
use serenity::http::{Http, Typing};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::model::prelude::MessageType;
use serenity::prelude::EventHandler;
use tracing::log::{error, info};

use crate::client::discord::EvHandler;
use crate::service::chat::chat_mode;
use crate::service::reply::reply_mode;

const ADMINISTRATOR: u64 = 586824421470109716;
// note: 暫定措置でBAN https://github.com/approvers/ichiyoAI/pull/53
const BANNED_USERS: &[u64] = &[
    596121630930108426, // yuzukiefu
    216216836214095872, // ksrg0
];

#[async_trait]
impl EventHandler for EvHandler {
    async fn message(&self, ctx: Context, new_msg: Message) {
        if new_msg.author.bot || new_msg.is_private() {
            return;
        }

        if BANNED_USERS.contains(&new_msg.author.id.0) {
            return;
        }

        if let Ok(false) = new_msg.mentions_me(&ctx).await {
            return;
        }

        let http = ctx.clone().http;
        let channel_id = new_msg.channel_id;

        info!(
            "{sender}: Started a conversation.",
            sender = new_msg.author.name
        );
        let typing = start_typing(http, channel_id);

        match new_msg.kind {
            // 通常メッセージ (チャットモード)
            MessageType::Regular => {
                if let Err(why) = chat_mode(&ctx, &new_msg).await {
                    let _ = new_msg
                        .reply(
                            &ctx,
                            &format!(
                                "Unexpected error reported! (Chat Mode), Read log <@{mention}> \n```{error}\n```",
                                mention = ADMINISTRATOR, error = why
                            ),
                        )
                        .await;
                    error!("{:?}", why)
                }
            }
            // 返信 (リプライモード)
            MessageType::InlineReply => {
                if let Err(why) = reply_mode(&ctx, &new_msg).await {
                    let _ = new_msg
                        .reply(
                            &ctx,
                            &format!(
                                "Unexpected error reported! (Reply Mode), Read log <@{mention}> \n```{error}\n```",
                                mention = ADMINISTRATOR, error = why
                            ),
                        )
                        .await;
                    error!("{:?}", why)
                }
            }
            _ => (),
        }

        typing.stop();
        info!(
            "{sender}: Conversation completed.",
            sender = new_msg.author.name
        )
    }

    async fn ready(&self, _: Context, self_bot: Ready) {
        info!(
            "Successfully connected to {username} (ID: {userid})",
            username = self_bot.user.name,
            userid = self_bot.user.id
        )
    }
}

fn start_typing(http: Arc<Http>, target_channel_id: ChannelId) -> Typing {
    Typing::start(http, u64::from(target_channel_id)).expect("Failed to start typing")
}
