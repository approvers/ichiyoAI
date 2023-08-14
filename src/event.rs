use crate::client::discord::EvHandler;
use crate::service::chat::chat_mode;
use crate::service::reply::reply_mode;
use chatgpt::prelude::ChatGPTEngine;
use serenity::async_trait;
use serenity::client::Context;
use serenity::http::{Http, Typing};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::model::prelude::{Activity, GuildId, MessageType, RoleId};
use serenity::prelude::EventHandler;
use std::sync::Arc;
use tracing::log::{error, info};

static VERSION: &str = env!("CARGO_PKG_VERSION");
static ADMINISTRATOR: u64 = 586824421470109716;
// 変わることはそうそうないので、定数化
static APPROVERS_ID: GuildId = GuildId(683939861539192860);
static SUBSCRIPTION_ROLE_ID: RoleId = RoleId(709699920730390559);

#[async_trait]
impl EventHandler for EvHandler {
    async fn message(&self, ctx: Context, new_msg: Message) {
        if new_msg.author.bot || new_msg.is_private() {
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

        let is_subscriber = new_msg
            .author
            .has_role(&ctx, APPROVERS_ID, SUBSCRIPTION_ROLE_ID)
            .await
            .unwrap();
        let model = if is_subscriber {
            ChatGPTEngine::Gpt4
        } else {
            ChatGPTEngine::Gpt35Turbo
        };

        match new_msg.kind {
            // 通常メッセージ (チャットモード)
            MessageType::Regular => {
                if let Err(why) = chat_mode(&ctx, &new_msg, model).await {
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
                if let Err(why) = reply_mode(&ctx, &new_msg, model).await {
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

    async fn ready(&self, ctx: Context, self_bot: Ready) {
        ctx.set_activity(Activity::playing(&format!("v{}", VERSION)))
            .await;

        info!(
            "Successfully connected to {username}! (ID: {userid}) - Using ichiyoAI v{version}",
            username = self_bot.user.name,
            userid = self_bot.user.id,
            version = VERSION
        )
    }
}

fn start_typing(http: Arc<Http>, target_channel_id: ChannelId) -> Typing {
    Typing::start(http, u64::from(target_channel_id)).expect("Failed to start typing")
}
