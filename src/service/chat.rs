use crate::api::openai::request_message;
use chatgpt::prelude::ChatGPTEngine::Gpt4;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub async fn chat_mode(ctx: &Context, msg: Message, content: String) -> anyhow::Result<()> {
    let response = request_message(content, Some(msg.clone().author.name), Some(Gpt4)).await?;
    let response_message = &response.message().content;

    msg.reply(ctx, response_message).await?;

    Ok(())
}
