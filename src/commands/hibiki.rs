use crate::api::chatgpt::chat_directed;
use crate::api::discord::{edit_response, reply};
use anyhow::Context as _;
use chatgpt::config::ChatGPTEngine;
use serenity::framework::standard::Args;
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use std::fs::File;
use std::io::Read;

const APPROVERS_HIBIKI_EMOJI: &str = "<:haracho:684424533997912096>";

pub async fn command_hibiki(ctx: &Context, msg: &Message, mut args: Args) -> anyhow::Result<()> {
    let content = args
        .single::<String>()
        .context("引数(1つ目)のパースに失敗しました")?;

    let waiting_message = reply(ctx, msg, &format!("思考中... {}", APPROVERS_HIBIKI_EMOJI)).await?;
    let settings = read_hibiki_settings_file()?;

    let response = chat_directed(&content, &settings, Some(ChatGPTEngine::Gpt35Turbo))
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

fn read_hibiki_settings_file() -> anyhow::Result<String> {
    let mut file = File::open("resource/hibiki-settings.txt")
        .context("響モードのファイルが見つかりませんでした。")?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .context("ファイルの読み込み中にエラーが発生しました。")?;

    Ok(content)
}
