use crate::client::openai::request_message;
use crate::model::{MessageCompletionResult, ReplyMessage, ReplyRole};
use anyhow::Ok;
use once_cell::sync::OnceCell;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use super::pricing::usage_pricing;

pub async fn chat_mode(ctx: &Context, msg: &Message, model: &str) -> anyhow::Result<()> {
    let reply = get_reply(ctx, msg).await?;
    let result = request_message(&reply, model).await?;

    msg.reply_ping(ctx, format_result(result, model)).await?;

    Ok(())
}

async fn get_reply(ctx: &Context, msg: &Message) -> anyhow::Result<Vec<ReplyMessage>> {
    static OWN_MENTION: OnceCell<String> = OnceCell::new();
    let mention = OWN_MENTION.get_or_init(|| format!("<@{}>", ctx.cache.current_user_id()));
    let content = msg.content.replace(mention, "").trim().to_string();

    if content.chars().count() < 5 {
        return Err(anyhow::anyhow!(
            "送信メッセージが短すぎます。5文字以上入力してください。"
        ));
    }

    let mut replies: Vec<ReplyMessage> = vec![ReplyMessage {
        role: ReplyRole::User,
        content,
    }];

    replies.reverse();
    Ok(replies)
}

fn format_result(result: MessageCompletionResult, model: &str) -> String {
    let pricing = usage_pricing(result.input_token, result.output_token, model);
    format!(
        "{}\n\n`利用料金: ￥{:.2}(合計トークン: {})` - `使用モデル: {}`",
        result.message, pricing, result.total_token, model
    )
}
