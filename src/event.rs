use crate::client::discord::EvHandler;
use crate::env::get_env;
use crate::service::chat::chat_mode;
use crate::service::reply::reply_mode;
use chatgpt::prelude::ChatGPTEngine;
use once_cell::sync::Lazy;
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
static GUILD_ID: Lazy<u64> = Lazy::new(|| get_env("GUILD_ID").parse().unwrap());
static SUBSCRIPTION_ROLE_ID: Lazy<u64> =
    Lazy::new(|| get_env("SUBSCRIPTION_ROLE_ID").parse().unwrap());

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

        info!("{sender}: 会話を開始します.", sender = new_msg.author.name);
        let typing = start_typing(http, channel_id);

        let is_subscriber = new_msg
            .author
            .has_role(&ctx, GuildId(*GUILD_ID), RoleId(*SUBSCRIPTION_ROLE_ID))
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
                        .reply_ping(
                            &ctx,
                            &format!("エラーが発生しました. \n```{error}\n```", error = why),
                        )
                        .await;
                    error!("{:?}", why)
                }
            }
            // 返信 (リプライモード)
            MessageType::InlineReply => {
                if let Err(why) = reply_mode(&ctx, &new_msg, model).await {
                    let _ = new_msg
                        .reply_ping(
                            &ctx,
                            &format!("エラーが発生しました.\n```{error}\n```", error = why),
                        )
                        .await;
                    error!("{:?}", why)
                }
            }
            _ => (),
        }

        typing.stop();
        info!(
            "{sender}: 会話を完了させました.",
            sender = new_msg.author.name
        )
    }

    async fn ready(&self, ctx: Context, self_bot: Ready) {
        ctx.set_activity(Activity::playing(&format!("v{}", VERSION)))
            .await;

        info!(
            "{username}(ID: {userid}) に接続しました! - ichiyoAI v{version} を使用しています.",
            username = self_bot.user.name,
            userid = self_bot.user.id,
            version = VERSION
        )
    }
}

fn start_typing(http: Arc<Http>, target_channel_id: ChannelId) -> Typing {
    Typing::start(http, u64::from(target_channel_id)).expect("タイピングを開始できませんでした.")
}
