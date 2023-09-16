use crate::client::openai::request_reply_message;
use crate::model::{MessageCompletionResult, ReplyMessage, ReplyRole};
use chatgpt::prelude::ChatGPTEngine;
use once_cell::sync::OnceCell;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use super::pricing::usage_pricing;

pub async fn reply_mode(ctx: &Context, msg: &Message, model: ChatGPTEngine) -> anyhow::Result<()> {
    let replies = get_replies(ctx, msg).await?;
    // notes: GPT-4 があまりにも高いため、GPT-3.5 に revert
    let result = request_reply_message(&replies, model).await?;

    msg.reply_ping(ctx, format_result(result, model)).await?;

    Ok(())
}

async fn get_replies(ctx: &Context, msg: &Message) -> anyhow::Result<Vec<ReplyMessage>> {
    static OWN_MENTION: OnceCell<String> = OnceCell::new();
    let mention = OWN_MENTION.get_or_init(|| format!("<@{}>", ctx.cache.current_user_id()));

    let mut replies: Vec<ReplyMessage> = vec![ReplyMessage {
        role: ReplyRole::User,
        content: msg.content.clone(),
    }];

    // TODO: イテレータにしたい
    let channel_id = msg.channel_id;
    let mut target_message_id = msg.referenced_message.as_ref().map(|m| m.id);
    while let Some(message_id) = target_message_id {
        // `.referenced_message`は直近のメッセージしかSome<T>では無いため，`.get_message`でメッセージを取得している．
        let message = ctx.http.get_message(channel_id.0, message_id.0).await?;

        let role = if message.is_own(ctx) {
            ReplyRole::Ichiyo
        } else {
            ReplyRole::User
        };

        let mut content = message.content.replace(mention, "").trim().to_string();

        // 一葉のメッセージの場合、最後の値段表示を削除する
        if role == ReplyRole::Ichiyo {
            let len = content.rfind("\n\n").unwrap_or(content.len());
            content.truncate(len);
        }

        let reply = ReplyMessage { role, content };

        replies.push(reply);

        target_message_id = message.referenced_message.map(|m| m.id);
    }

    replies.reverse();
    Ok(replies)
}

// chatにあるものと同一だが、変更の可能性が高いためあえて共通化しない
fn format_result(result: MessageCompletionResult, model: ChatGPTEngine) -> String {
    let pricing = usage_pricing(result.input_token, result.output_token, model);
    format!(
        "{}\n\n`利用料金: ￥{:.2}(合計トークン: {})` - `使用モデル: {}`",
        result.message,
        pricing,
        result.total_token,
        model.to_string()
    )
}
