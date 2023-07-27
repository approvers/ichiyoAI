use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::api::chatgpt::chat_completion;
use crate::api::discord::{edit_response, send_message};

pub async fn chat_ai(ctx: &Context, msg: Message) {
    let waiting_message = send_message(ctx, &msg, "思考中... 🤔").await;

    let response = chat_completion(&msg.content).await;
    let response_content = &response.message().content;

    match response_content.chars().count() {
        count if count > 2000 => {
            send_message(
                ctx,
                &msg,
                "レスポンスが2000文字を超えたため、表示できません。",
            )
            .await;
        }
        _ => edit_response(&ctx, waiting_message, response_content).await,
    }
}
