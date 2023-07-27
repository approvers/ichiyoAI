use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub async fn send_message(ctx: &Context, msg: &Message, content: &str) -> Message {
    msg.reply_ping(ctx, content)
        .await
        .expect("メッセージの送信に失敗しました。")
}

pub async fn edit_response(ctx: &Context, mut target_message: Message, response: &String) {
    target_message
        .edit(ctx, |m| m.content(response))
        .await
        .expect("メッセージの編集に失敗しました。");
}
