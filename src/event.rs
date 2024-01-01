use crate::adapters::message::start_typing;
use crate::adapters::{
    message::{push_referenced_msg_to_prompts, reply_chatgpt_response},
    user::is_sponsor,
};
use crate::model::env::ICHIYOAI_ENV;
use ichiyo_ai::Completion as _;
use ichiyo_ai::{OpenAiGPT35Turbo, OpenAiGPT4Turbo};
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

        let mention = OWN_MENTION.get_or_init(|| format!("<@{}>", ctx.cache.current_user().id));
        let content = msg.content.replace(mention, "");
        if content.chars().count() == 0 {
            return;
        }

        let mut messages = Vec::new();
        messages.push(ichiyo_ai::Message::User {
            id: (),
            content: SYSTEM_CONTEXT.to_owned(),
        });

        messages.push({
            match msg.is_own(&ctx) {
                true => ichiyo_ai::Message::Model { id: (), content },
                false => ichiyo_ai::Message::User { id: (), content },
            }
        });

        if msg.kind == MessageType::InlineReply {
            let Some(referenced_id) = msg.referenced_message.as_ref().map(|m| m.id) else {
                panic!();
            };

            if let Err(why) =
                push_referenced_msg_to_prompts(&ctx, &mut messages, msg.channel_id, referenced_id)
                    .await
            {
                error!("Failed to push referenced message to prompts: {}", why);
                return;
            }
        }

        messages.reverse();

        let is_sponsor = match is_sponsor(&ctx, &msg.author).await {
            Ok(is_sponsor) => is_sponsor,
            Err(why) => {
                error!("Failed to check sponsor: {}", why);
                false
            }
        };

        use anyhow::Context as _;

        let token = ICHIYOAI_ENV
            .get()
            .expect("Failed to get openai api key from env")
            .openai_api_key
            .as_str();

        let typing = start_typing(ctx.http.clone(), msg.channel_id);

        let result = match is_sponsor {
            true => {
                let engine = OpenAiGPT4Turbo::new(token);
                tokio::time::timeout(crate::adapters::TIMEOUT_DURATION, engine.next(&messages))
                    .await
                    .context("Operation timed out")
            }
            false => {
                let engine = OpenAiGPT35Turbo::new(token);
                tokio::time::timeout(crate::adapters::TIMEOUT_DURATION, engine.next(&messages))
                    .await
                    .context("Operation timed out")
            }
        };

        typing.stop();

        let (response, metadata) = match result {
            Ok(Ok(response)) => response,
            Ok(Err(why)) | Err(why) => {
                let _ = msg
                    .reply_ping(&ctx, format!("An error has occurred: {}", why))
                    .await;
                error!("Failed to request chatgpt response: {}", why);
                return;
            }
        };

        reply_chatgpt_response(&ctx, &msg, response.content(), &metadata)
            .await
            .unwrap();
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
