use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub async fn reply(ctx: &Context, msg: &Message, content: &str) -> Message {
    msg.reply_ping(ctx, content)
        .await
        .expect("メッセージの送信に失敗しました。")
}

pub async fn edit_response(ctx: &Context, mut target_message: Message, response: &str) {
    target_message
        .edit(ctx, |m| m.content(response))
        .await
        .expect("メッセージの編集に失敗しました。");
}

pub async fn edit_response_with_file(ctx: &Context, mut target_msg: Message, file_name: &str) {
    target_msg
        .edit(ctx, |m| {
            m.content("レスポンスが2000文字を超えたため、添付ファイルで表示します。")
        })
        .await
        .expect("メッセージの編集に失敗しました。");
    target_msg
        .channel_id
        .send_message(ctx, |m| m.add_file(file_name))
        .await
        .expect("メッセージの送信に失敗しました。");
}
