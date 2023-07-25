use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub async fn reply_response(ctx: &Context, msg: Message, response: &String) {
    let mut m = msg
        .reply(ctx, "思考中... 🤔")
        .await
        .expect("メッセージの送信に失敗しました。");
    m.edit(ctx, |m| m.content(response))
        .await
        .expect("メッセージの編集に失敗しました。");
}
