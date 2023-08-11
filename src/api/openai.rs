use crate::env::get_env;
use anyhow::{ensure, Context};
use chatgpt::config::ModelConfigurationBuilder;
use chatgpt::prelude::{ChatGPT, ChatGPTEngine};
use chatgpt::types::CompletionResponse;
use once_cell::sync::Lazy;

static OPENAI_API_KEY: Lazy<String> = Lazy::new(|| get_env("OPENAI_API_KEY"));

/// OpenAI API のクライアントを初期化します。
///
/// ### 引数
/// * `api_key` -- OpenAI API の API Key を指定する。
/// * `model` --
///         使用する ChatGPT のモデルを使用する。使用できるモデルは [ChatGPTEngine] で定義されている物のみ。
///         指定しない場合([None])は [ChatGPTEngine::Gpt35Turbo] が使用される。
///
/// ### 返り値
/// [ChatGPT]: OpenAI API (ChatGPT) のクライアント
///
/// ### エラー
/// * クライアントの初期化に失敗した際 [anyhow::Result] により、エラーが報告されます。
fn init_client(api_key: &str, model: Option<ChatGPTEngine>) -> anyhow::Result<ChatGPT> {
    let use_engine = model.unwrap_or(ChatGPTEngine::Gpt35Turbo);
    let client = ChatGPT::new_with_config(
        api_key,
        ModelConfigurationBuilder::default()
            .engine(use_engine)
            .build()
            .unwrap(),
    )
    .context("Failed to initialize OpenAI API")?;

    Ok(client)
}

/// ChatGPT に対してメッセージを送信し、レスポンスをリクエストします。
///
/// ### 引数
/// * `content` -- ChatGPT に送信するメッセージ
///
/// ### 返り値
/// [CompletionResponse]: ChatGPT からのレスポンス
///
/// ### エラー
/// 下記条件でエラーが報告されます。
/// * ChatGPT とのやり取りに失敗する
/// * 2000文字を超過する
pub async fn request_message(content: String) -> anyhow::Result<CompletionResponse> {
    let client = init_client(OPENAI_API_KEY.as_str(), None)?;
    let response = client
        .send_message(content)
        .await
        .context("Failed to communicate with ChatGPT")?;

    ensure!(
        &response.message().content.len() <= &2000,
        "Message response exceeded 2000 characters."
    );
    Ok(response)
}
