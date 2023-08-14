use chatgpt::prelude::ChatGPTEngine;
use chatgpt::types::CompletionResponse;
use once_cell::sync::OnceCell;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::client::openai::request_message;

pub async fn chat_mode(ctx: &Context, msg: &Message, model: ChatGPTEngine) -> anyhow::Result<()> {
    let response = get_response(ctx, msg, model).await?;
    let reply_content = &response.message().content;

    msg.reply(ctx, reply_content).await?;

    Ok(())
}

async fn get_response(
    ctx: &Context,
    msg: &Message,
    model: ChatGPTEngine,
) -> anyhow::Result<CompletionResponse> {
    static OWN_MENTION: OnceCell<String> = OnceCell::new();
    let mention = OWN_MENTION.get_or_init(|| format!("<@{}>", ctx.cache.current_user_id()));

    let content = msg.content.replace(mention, "").trim().to_string();

    let response = request_message(content, model).await?;

    Ok(response)
}
