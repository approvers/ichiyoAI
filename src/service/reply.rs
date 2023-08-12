use crate::api::openai::request_reply_message;
use crate::model::{ReplyMessage, ReplyRole};
use chatgpt::prelude::ChatGPTEngine::Gpt4;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub async fn reply_mode(ctx: &Context, msg: &Message) -> anyhow::Result<()> {
    let replies = get_replies(msg);
    let response_message = request_reply_message(&replies, Some(Gpt4)).await?;

    msg.reply(ctx, response_message).await?;

    Ok(())
}

fn get_replies(msg: &Message) -> Vec<ReplyMessage> {
    let mut replies: Vec<ReplyMessage> = vec![ReplyMessage {
        role: ReplyRole::User,
        content: msg.content.clone(),
    }];

    // TODO: イテレータにしたい
    let mut target_message = &msg.referenced_message;
    while let Some(ref message) = target_message {
        let role = if message.author.bot {
            ReplyRole::Ichiyo
        } else {
            ReplyRole::User
        };

        let reply = ReplyMessage {
            role,
            content: message.content.clone(),
        };

        replies.push(reply);

        target_message = &message.referenced_message;
    }

    replies.reverse();
    replies
}
