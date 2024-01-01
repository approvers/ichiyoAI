use std::sync::Arc;

use anyhow::{Context as _, Ok};
use serenity::{
    all::ChannelId,
    all::MessageId,
    client::Context,
    http::{Http, Typing},
    model::channel::Message,
};

use crate::event::OWN_MENTION;

pub fn start_typing(http: Arc<Http>, channel_id: ChannelId) -> Typing {
    Typing::start(http, channel_id)
}

pub async fn push_referenced_msg_to_prompts(
    ctx: &Context,
    messages: &mut Vec<ichiyo_ai::Message<()>>,
    channel_id: ChannelId,
    referenced_id: MessageId,
) -> anyhow::Result<()> {
    let mention = OWN_MENTION.get_or_init(|| format!("<@{}>", ctx.cache.current_user().id));

    let mut next = Some(referenced_id);
    while let Some(message_id) = next {
        let msg = ctx
            .http
            .clone()
            .get_message(channel_id, message_id)
            .await
            .context("Failed to get referenced message")?;

        let mut content = msg.content.replace(mention, "").trim().to_string();
        let is_own = msg.is_own(ctx);

        if is_own {
            let len = content.rfind("\n\n").unwrap_or(content.len());
            content.truncate(len);
        }

        messages.push({
            match is_own {
                true => ichiyo_ai::Message::Model { id: (), content },
                false => ichiyo_ai::Message::User { id: (), content },
            }
        });

        next = msg.referenced_message.as_ref().map(|m| m.id);
    }

    Ok(())
}

pub async fn reply_chatgpt_response(
    ctx: &Context,
    msg: &Message,
    content: impl core::fmt::Display,
    metadata: &impl ichiyo_ai::Metadata,
) -> anyhow::Result<()> {
    let price_yen = metadata.price_yen();
    let tokens = metadata.tokens();
    let model = metadata.by();

    let content = format!("{content}\n\n{model}￤¥{price_yen:.2}￤{tokens} tokens");

    let _ = msg
        .reply_ping(&ctx, content)
        .await
        .context("Failed to reply.");

    Ok(())
}
