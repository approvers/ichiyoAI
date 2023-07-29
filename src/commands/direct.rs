use crate::api::chatgpt::chat_directed;
use crate::api::discord::{edit_response, reply};
use anyhow::Context as _;
use chatgpt::config::ChatGPTEngine;
use serenity::framework::standard::Args;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub async fn command_direct(ctx: &Context, msg: &Message, mut args: Args) -> anyhow::Result<()> {
    let indication = args
        .single::<String>()
        .context("引数(1つ目)のパースに失敗しました。")?;
    let content = args
        .single::<String>()
        .context("引数(2つ目)のパースに失敗しました。")?;

    let waiting_message = reply(
        ctx,
        msg,
        &format!("思考中... 🤔\n**指示内容:**\n```{}\n```", indication),
    )
    .await?;

    // TODO: GPT-4に対応する
    let response = chat_directed(&content, &indication, Some(ChatGPTEngine::Gpt35Turbo))
        .await
        .context("ChatGPT APIとのやり取りに失敗しました。")?;

    let response_content = response.message().content.as_str();

    match response_content.chars().count() {
        count if count > 2000 => {
            edit_response(
                ctx,
                waiting_message,
                "レスポンスが2000文字を超えたため表示できません。",
            )
            .await?;
        }
        _ => edit_response(ctx, waiting_message, response_content).await?,
    }

    Ok(())
}
