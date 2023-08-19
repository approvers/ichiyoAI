use anyhow::Ok;
use chatgpt::prelude::ChatGPTEngine;
use once_cell::sync::OnceCell;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::client::openai::request_message;
use crate::model::{ReplyMessage, ReplyRole};

pub async fn chat_mode(ctx: &Context, msg: &Message, model: ChatGPTEngine) -> anyhow::Result<()> {
    let reply = get_reply(ctx, msg).await?;
    let result = request_message(&reply, model).await?;

    msg.reply_ping(ctx, result.message).await?;

    Ok(())
}

async fn get_reply(ctx: &Context, msg: &Message) -> anyhow::Result<Vec<ReplyMessage>> {
    static OWN_MENTION: OnceCell<String> = OnceCell::new();
    let mention = OWN_MENTION.get_or_init(|| format!("<@{}>", ctx.cache.current_user_id()));
    let content = msg.content.replace(mention, "").trim().to_string();

    let mut replies: Vec<ReplyMessage> = vec![ReplyMessage {
        role: ReplyRole::User,
        content,
    }];

    replies.reverse();
    Ok(replies)
}
