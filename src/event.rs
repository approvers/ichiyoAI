use crate::adapters::chatgpt::{request_chatgpt_message, SYSTEM_CONTEXT};
use crate::adapters::discord::{format_result, reply_completion_result};
use crate::model::chatgpt::RequestMessageModel;
use crate::model::discord::DiscordReplyMessageModel;
use crate::model::env::ICHIYOAI_ENV;
use crate::model::EvHandler;
use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs, Role};
use once_cell::sync::OnceCell;
use serenity::async_trait;
use serenity::client::Context;
use serenity::http::{Http, Typing};
use serenity::model::channel::{Message, MessageType};
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, GuildId, RoleId};
use serenity::model::prelude::Activity;
use serenity::prelude::EventHandler;
use std::sync::Arc;
use tracing::debug;
use tracing::log::{error, info};

#[async_trait]
impl EventHandler for EvHandler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.author.bot || message.is_private() {
            return;
        }

        if let Ok(false) = message.mentions_me(&ctx).await {
            return;
        }

        match process_ichiyoai(ctx, message).await {
            Ok(()) => (),
            Err(why) => error!("Processing message failed with error: {}", why),
        }
    }

    async fn ready(&self, ctx: Context, self_bot: Ready) {
        info!("Starting...");

        let version = env!("CARGO_PKG_VERSION");
        ctx.set_activity(Activity::playing(&format!("v{}", version)))
            .await;

        info!("Running ichiyoAI v{}", version);
        info!(
            "Connected!: {name}(Id:{id})",
            name = self_bot.user.name,
            id = self_bot.user.id
        )
    }
}

async fn process_ichiyoai(ctx: Context, message: Message) -> anyhow::Result<()> {
    static OWN_MENTION: OnceCell<String> = OnceCell::new();
    let channel_id = message.channel_id;
    let mention = OWN_MENTION.get_or_init(|| format!("<@{}>", ctx.cache.current_user_id()));
    let content = message.content.replace(mention, "").trim().to_string();

    if content.chars().count() < 5 {
        return Err(anyhow::anyhow!(
            "Message is too short. Please enter at least 5 characters."
        ));
    }

    let typing = start_typing(ctx.http.clone(), channel_id);
    let is_subscriber = message
        .author
        .has_role(
            &ctx,
            GuildId(ICHIYOAI_ENV.get().unwrap().guild_id),
            RoleId(ICHIYOAI_ENV.get().unwrap().taxpayer_role_id),
        )
        .await
        .unwrap_or(false);
    let model = if is_subscriber {
        // NOTE: preview model. See also: https://platform.openai.com/docs/models/gpt-4-and-gpt-4-turbo
        "gpt-4-1106-preview".to_string()
        // "gpt-4".to_string()
    } else {
        "gpt-3.5-turbo".to_string()
    };
    let mut replies: Vec<ChatCompletionRequestMessage> =
        vec![ChatCompletionRequestMessageArgs::default()
            .role(Role::System)
            .content(SYSTEM_CONTEXT)
            .build()?];

    match message.kind {
        MessageType::Regular => {
            let reply = ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(content)
                .build()?;
            replies.push(reply);
        }
        MessageType::InlineReply => {
            let reply = ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(content)
                .build()?;
            replies.push(reply);

            let mut target_message_id = message.referenced_message.as_ref().map(|m| m.id);
            while let Some(message_id) = target_message_id {
                let ref_message = ctx
                    .http
                    .clone()
                    .get_message(channel_id.0, message_id.0)
                    .await?;

                let role = if ref_message.is_own(ctx.clone()) {
                    Role::Assistant
                } else {
                    Role::User
                };

                let mut content = ref_message.content.replace(mention, "").trim().to_string();

                if role == Role::Assistant {
                    let len = content.rfind("\n\n").unwrap_or(content.len());
                    content.truncate(len);
                }

                let reply = ChatCompletionRequestMessageArgs::default()
                    .role(role)
                    .content(content)
                    .build()?;

                replies.push(reply);
                target_message_id = ref_message.referenced_message.map(|m| m.id);
            }
        }
        _ => (),
    }

    replies.reverse();
    debug!("{:?}", replies.iter());

    let request = RequestMessageModel::builder()
        .replies(replies)
        .model(model.clone())
        .build();
    let result = request_chatgpt_message(request).await?;

    let reply = DiscordReplyMessageModel::builder()
        .http(ctx.http.clone())
        .target_message(message.clone())
        .formatted_result(format_result(result, &model))
        .build();

    if let Err(why) = reply_completion_result(reply).await {
        error!("{}", why)
    }
    typing.stop();

    Ok(())
}

fn start_typing(http: Arc<Http>, target_channel_id: ChannelId) -> Typing {
    Typing::start(http, u64::from(target_channel_id)).expect("タイピングを開始できませんでした.")
}
