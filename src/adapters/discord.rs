use crate::model::chatgpt::{usage_pricing, ResponseCompletionResultModel};
use crate::model::discord::DiscordReplyMessageModel;
use anyhow::Context;

pub async fn reply_completion_result(
    reply_message: DiscordReplyMessageModel,
) -> anyhow::Result<()> {
    reply_message
        .target_message
        .reply_ping(reply_message.http, reply_message.formatted_result)
        .await
        .context("Failed to reply.")?;

    Ok(())
}

pub fn format_result(result: ResponseCompletionResultModel, model: &str) -> String {
    let pricing = usage_pricing(result.input_token, result.output_token, model);
    format!(
        "{}\n\n`利用料金: ￥{:.2}` - `合計トークン: {}` - `使用モデル: {}`",
        result.response_message, pricing, result.total_token, model
    )
}
