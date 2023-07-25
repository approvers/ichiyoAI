use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::api::chatgpt::chat_completion;
use crate::api::discord::reply_response;

pub async fn chat_ai(ctx: &Context, msg: Message) {
    let response = chat_completion(&msg.content).await;
    let response_content = &response.message().content;

    match response_content.chars().count() {
        count if count > 2000 => {
            // TODO: 添付ファイルにして表示できるようにする
            msg.reply(&ctx, "レスポンスが2000文字を超えたため、表示できません。")
                .await
                .expect("メッセージの送信に失敗しました。");
        }
        _ => reply_response(&ctx, msg, response_content).await,
    }
}
