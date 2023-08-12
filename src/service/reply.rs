use crate::api::openai::request_reply_message;
use chatgpt::prelude::ChatGPTEngine::Gpt4;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub struct ReplyMessages {
    pub before_message: String,
    pub after_message: String,
}

impl ReplyMessages {
    pub fn new(before_message: String, after_message: String) -> Self {
        ReplyMessages {
            before_message,
            after_message,
        }
    }
}

pub async fn reply_mode(ctx: &Context, msg: Message) -> anyhow::Result<()> {
    let replies = spilt_message(&msg)?;
    let response_message =
        request_reply_message(replies, Some(msg.clone().author.name), Some(Gpt4)).await?;

    msg.reply(ctx, response_message).await?;

    Ok(())
}

fn spilt_message(msg: &Message) -> anyhow::Result<ReplyMessages> {
    let before_message = match &msg.referenced_message {
        Some(message) => &message.content,
        None => {
            return Err(anyhow::anyhow!(
                "The message to which you are replying could not be found."
            ))
        }
    };

    Ok(ReplyMessages::new(
        before_message.clone(),
        msg.clone().content,
    ))
}
