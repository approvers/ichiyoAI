use crate::env::get_env;
use crate::model::{ReplyMessage, ReplyRole};
use anyhow::{ensure, Context};
use chatgpt::config::ModelConfigurationBuilder;
use chatgpt::prelude::{ChatGPT, ChatGPTEngine};
use chatgpt::types::{ChatMessage, CompletionResponse, Role};
use once_cell::sync::Lazy;
use std::time::Duration;
use tokio::time::timeout;

static TIMEOUT_DURATION: Duration = Duration::from_secs(60);
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
/// * `model` --
///         使用する ChatGPT のモデルを使用する。使用できるモデルは [ChatGPTEngine] で定義されている物のみ。
///         指定しない場合([None])は [ChatGPTEngine::Gpt35Turbo] が使用される。
/// ### 返り値
/// [CompletionResponse]: ChatGPT からのレスポンス
///
/// ### エラー
/// 下記条件でエラーが報告されます。
/// * ChatGPT とのやり取りに失敗する
/// * 2000文字を超過する
pub async fn request_message(
    content: String,
    model: ChatGPTEngine,
) -> anyhow::Result<CompletionResponse> {
    let client = init_client(OPENAI_API_KEY.as_str(), Some(model))?;

    let response = match timeout(TIMEOUT_DURATION, client.send_message(content)).await {
        Ok(result) => result.context("Failed to communicate with ChatGPT")?,
        Err(_) => return Err(anyhow::anyhow!("Operation timed out.")),
    };

    ensure!(
        &response.message().content.len() <= &2000,
        "Message response exceeded 2000 characters."
    );
    Ok(response)
}

/// ChatGPT に対して一連の会話コンテキストを送信し、レスポンスをリクエストします。
///
/// ### 引数
/// * `messages` -- ChatGPT　に送信する会話コンテキスト。[ReplyMessages] を実装しておく必要がある。
/// * `model` --
///         使用する ChatGPT のモデルを使用する。使用できるモデルは [ChatGPTEngine] で定義されている物のみ。
///         指定しない場合([None])は [ChatGPTEngine::Gpt35Turbo] が使用される。
///
/// ### 返り値
/// [String]: ChatGPT からのレスポンス
pub async fn request_reply_message(
    reply_messages: &[ReplyMessage],
    model: ChatGPTEngine,
) -> anyhow::Result<String> {
    let client = init_client(OPENAI_API_KEY.as_str(), Some(model))?;

    let history = reply_messages
        .iter()
        .map(|reply| ChatMessage {
            content: reply.content.clone(),
            role: match reply.role {
                ReplyRole::Ichiyo => Role::Assistant,
                ReplyRole::User => Role::User,
            },
        })
        .collect::<Vec<ChatMessage>>();

    let response = timeout(TIMEOUT_DURATION, client.send_history(&history))
        .await
        .context("Operation timed out.")??;

    let response_message = response.message().content.clone();

    Ok(response_message)
}
