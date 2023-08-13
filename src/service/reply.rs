use chatgpt::prelude::ChatGPTEngine::Gpt4;
use once_cell::sync::OnceCell;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::client::openai::request_reply_message;
use crate::model::{ReplyMessage, ReplyRole};

pub async fn reply_mode(ctx: &Context, msg: &Message) -> anyhow::Result<()> {
    let replies = get_replies(ctx, msg).await?;
    let response_message = request_reply_message(&replies, Some(Gpt4)).await?;

    msg.reply(ctx, response_message).await?;

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

        let content = message.content.replace(mention, "").trim().to_string();

        let reply = ReplyMessage { role, content };

        replies.push(reply);

        target_message_id = message.referenced_message.map(|m| m.id);
    }

    replies.reverse();
    Ok(replies)
}
