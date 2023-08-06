use anyhow::Context as _;
use chatgpt::config::ChatGPTEngine;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::api::chatgpt::chat_completion;
use crate::api::discord::{edit_response, reply};

pub async fn chat(ctx: &Context, msg: &Message, request_content: &str) -> anyhow::Result<()> {
    let waiting_message = reply(ctx, msg, "思考中... 🤔").await?;

    let response = chat_completion(request_content, Some(ChatGPTEngine::Gpt35Turbo))
        .await
        .context("ChatGPT APIとのやり取りに失敗しました。")?;

    let response_content = &response.message().content;

    match response_content.chars().count() {
        count if count > 2000 => {
            edit_response(
                ctx,
                waiting_message,
                "レスポンスが2000文字を超えたため表示できません。",
            )
            .await?
        }
        _ => edit_response(ctx, waiting_message, response_content).await?,
    }

    Ok(())
}
