use std::env;

use chatgpt::prelude::ChatGPT;
use chatgpt::types::CompletionResponse;
use once_cell::sync::Lazy;

static CHATGPT_API_TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("CHATGPT_API_TOKEN").expect("Expected a token in the environment (CHATGPT_API_TOKEN)")
});

pub async fn chat_completion(message: &String) -> CompletionResponse {
    let client =
        ChatGPT::new(&*CHATGPT_API_TOKEN).expect("ChatGPI APIのクライアント初期化に失敗しました。");

    client
        .send_message(message)
        .await
        .expect("ChatGPI APIとのチャットに失敗しました。")
}
