use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestUserMessageArgs};
use once_cell::sync::OnceCell;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    gateway::ActivityData,
    model::channel::Message,
    model::{channel::MessageType, gateway::Ready},
};
use std::result::Result::Ok;
use tracing::{error, info};

use crate::adapters::{
    chatgpt::request_chatgpt_response,
    message::{push_referenced_msg_to_prompts, reply_chatgpt_response},
    user::is_sponsor,
};

pub struct EvHandler;

pub static SYSTEM_CONTEXT: &str = "回答時は以下のルールに従うこと.\n- 1900文字以内に収めること。";
pub static OWN_MENTION: OnceCell<String> = OnceCell::new();

#[async_trait]
impl EventHandler for EvHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.is_private() {
            return;
        }

        if let Ok(false) = msg.mentions_me(&ctx).await {
            return;
        }

        let mention = OWN_MENTION.get_or_init(|| format!("<@!{}>", ctx.cache.current_user().id));
        let content = msg.content.replace(mention, "");
        let mut prompts: Vec<ChatCompletionRequestMessage> =
            vec![ChatCompletionRequestUserMessageArgs::default()
                .content(SYSTEM_CONTEXT)
                .build()
                .unwrap()
                .into()];

        if content.chars().count() == 0 {
            return;
        }

        prompts.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(content)
                .build()
                .unwrap()
                .into(),
        );

        if msg.kind == MessageType::InlineReply {
            if let Err(why) = push_referenced_msg_to_prompts(&mut prompts, &ctx, &msg).await {
                error!("Failed to push referenced message to prompts: {}", why);
                return;
            }
        }

        let is_gpt4 = match is_sponsor(&ctx, msg.clone().author).await {
            Ok(is_gpt4) => is_gpt4,
            Err(why) => {
                error!("Failed to check sponsor: {}", why);
                false
            }
        };

        prompts.reverse();

        let response = match request_chatgpt_response(&ctx, msg.channel_id, prompts, is_gpt4).await
        {
            Ok(response) => response,
            Err(why) => {
                let _ = msg
                    .reply_ping(&ctx, format!("An error has occurred: {}", why))
                    .await;
                error!("Failed to request chatgpt response: {}", why);
                return;
            }
        };

        reply_chatgpt_response(response, &ctx, &msg).await.unwrap();
    }

    async fn ready(&self, ctx: Context, self_bot: Ready) {
        info!("Starting...");

        let version = env!("CARGO_PKG_VERSION");
        ctx.set_activity(Some(ActivityData::playing(&format!("v{}", version))));

        info!("Running ichiyoAI v{}", version);
        info!(
            "Connected!: {name}(Id:{id})",
            name = self_bot.user.name,
            id = self_bot.user.id
        )
    }
}
