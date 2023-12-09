use std::sync::Arc;

use anyhow::{Context as _, Ok};
use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestUserMessageArgs, Role,
};
use serenity::{
    all::ChannelId,
    client::Context,
    http::{Http, Typing},
    model::channel::Message,
};

use crate::{
    event::OWN_MENTION,
    model::chatgpt::{usage_pricing, ChatGPTResponseModel},
};

pub fn start_typing(http: Arc<Http>, channel_id: ChannelId) -> Typing {
    Typing::start(http, channel_id)
}

pub async fn push_referenced_msg_to_prompts(
    prompts: &mut Vec<ChatCompletionRequestMessage>,
    ctx: &Context,
    msg: &Message,
) -> anyhow::Result<()> {
    let mention = OWN_MENTION.get_or_init(|| format!("<@{}>", ctx.cache.current_user().id));
    let channel_id = msg.channel_id;
    let mut target_message_id = msg.referenced_message.as_ref().map(|m| m.id);

    while let Some(message_id) = target_message_id {
        let ref_msg = ctx
            .http
            .clone()
            .get_message(channel_id, message_id)
            .await
            .context("Failed to get referenced message")?;

        let role = if ref_msg.is_own(ctx) {
            Role::Assistant
        } else {
            Role::User
        };

        let mut content = ref_msg.content.replace(mention, "").trim().to_string();

        match role {
            Role::Assistant => {
                let len = content.rfind("\n\n").unwrap_or(content.len());
                content.truncate(len);

                prompts.push(
                    ChatCompletionRequestAssistantMessageArgs::default()
                        .content(content)
                        .build()?
                        .into(),
                )
            }
            Role::User => prompts.push(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(content)
                    .build()?
                    .into(),
            ),
            _ => {}
        }

        target_message_id = ref_msg.referenced_message.as_ref().map(|m| m.id);
    }

    Ok(())
}

pub async fn reply_chatgpt_response(
    res: ChatGPTResponseModel,
    ctx: &Context,
    msg: &Message,
) -> anyhow::Result<()> {
    let usage = usage_pricing(res.prompt_tokens, res.completion_tokens, &res.model);
    let _ = msg
        .reply_ping(
            &ctx,
            format!(
                "{}\n\n{}￤¥{:.2}￤{} tokens",
                res.res, res.model, usage, res.total_tokens
            ),
        )
        .await
        .context("Failed to reply.");

    Ok(())
}
