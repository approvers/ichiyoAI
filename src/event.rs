use crate::client::discord::EvHandler;
use crate::service::chat::chat_mode;
use serenity::async_trait;
use serenity::client::Context;
use serenity::http::{Http, Typing};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, UserId};
use serenity::model::prelude::MessageType;
use serenity::prelude::EventHandler;
use std::sync::Arc;
use tracing::log::{error, info};

const ADMINISTRATOR: u64 = 586824421470109716;

#[async_trait]
impl EventHandler for EvHandler {
    async fn message(&self, ctx: Context, new_msg: Message) {
        if new_msg.author.bot || new_msg.is_private() {
            return;
        }

        info!(
            "{sender}: Started a conversation.",
            sender = new_msg.author.name
        );

        let channel_id = new_msg.channel_id;
        let self_user_id = ctx.clone().cache.current_user_id();

        match new_msg.kind {
            // 通常メッセージ (チャットモード)
            MessageType::Regular => {
                let typing = start_typing(ctx.clone().http, channel_id);
                let content = remove_mention(&new_msg.content, self_user_id);

                if let Err(why) = chat_mode(&ctx, new_msg.clone(), content).await {
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

                typing.stop();
            }
            _ => (),
        }

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

fn remove_mention(target_msg_content: &str, current_user_id: UserId) -> String {
    target_msg_content.replace(&format!("<@{current_user_id}>"), "")
}
