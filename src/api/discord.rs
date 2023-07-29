use anyhow::Context as _;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub async fn reply(ctx: &Context, msg: &Message, content: &str) -> anyhow::Result<Message> {
    msg.reply_ping(ctx, content)
        .await
        .context("メッセージの送信に失敗しました。")
}

pub async fn edit_response(
    ctx: &Context,
    mut target_message: Message,
    response: &str,
) -> anyhow::Result<()> {
    target_message
        .edit(ctx, |m| m.content(response))
        .await
        .context("メッセージの編集に失敗しました。")
}
