use crate::env::get_env;
use crate::model::{MessageCompletionResult, ReplyMessage, ReplyRole};
use anyhow::{Context, Ok};
use chatgpt::config::ModelConfigurationBuilder;
use chatgpt::prelude::{ChatGPT, ChatGPTEngine};
use chatgpt::types::{ChatMessage, Role};
use once_cell::sync::Lazy;
use std::time::Duration;
use tokio::time::timeout;

static TIMEOUT_DURATION: Duration = Duration::from_secs(180);
static OPENAI_API_KEY: Lazy<String> = Lazy::new(|| get_env("OPENAI_API_KEY"));
// 会話モード・返信モード で使用するシステムコンテキスト。膨大なレスポンスにならないように抑える目的に使用する。
// レスポンスの後にメタ情報（利用料金表示など）を含めるため、100字分の余裕を設けている。
static SYSTEM_CONTEXT: &str = "回答時は以下のルールに従うこと.\n- 1900文字以内に収めること。\n- なるべく簡潔に言うこと。\n- 一般的に知られている単語は説明しない。";

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
    .context("OpenAI API クライアントの初期化に失敗しました.")?;

    Ok(client)
}

/// ChatGPT に対してメッセージを送信し、レスポンスをリクエストします。
///
/// ### 引数
/// * `request_message` -- ChatGPT に送信するメッセージ。[ReplyMessages] を実装しておく必要がある。
/// * `model` --
///         使用する ChatGPT のモデルを使用する。使用できるモデルは [ChatGPTEngine] で定義されている物のみ。
///         指定しない場合([None])は [ChatGPTEngine::Gpt35Turbo] が使用される。
/// ### 返り値
/// [String]: ChatGPT からのレスポンス
///
/// ### エラー
/// 下記条件でエラーが報告されます。
/// * ChatGPT とのやり取りに失敗する
/// * 2000文字を超過する
pub async fn request_message(
    request_message: &[ReplyMessage],
    model: ChatGPTEngine,
) -> anyhow::Result<MessageCompletionResult> {
    let client = init_client(OPENAI_API_KEY.as_str(), Some(model))?;

    let mut history = request_message
        .iter()
        .map(|reply| ChatMessage {
            content: reply.content.clone(),
            role: Role::User,
        })
        .collect::<Vec<ChatMessage>>();

    history.insert(
        0,
        ChatMessage {
            role: Role::System,
            content: SYSTEM_CONTEXT.to_string(),
        },
    );

    let response = timeout(TIMEOUT_DURATION, client.send_history(&history))
        .await
        .context("タイムアウトしました, もう一度お試しください.")??;

    let result = MessageCompletionResult {
        message: response.message().content.clone(),
        input_token: response.usage.prompt_tokens,
        output_token: response.usage.completion_tokens,
    };

    Ok(result)
}

/// ChatGPT に対して一連の会話コンテキストを送信し、レスポンスをリクエストします。
///
/// ### 引数
/// * `reply_messages` -- ChatGPT　に送信する会話コンテキスト。[ReplyMessages] を実装しておく必要がある。
/// * `model` --
///         使用する ChatGPT のモデルを使用する。使用できるモデルは [ChatGPTEngine] で定義されている物のみ。
///         指定しない場合([None])は [ChatGPTEngine::Gpt35Turbo] が使用される。
///
/// ### 返り値
/// [String]: ChatGPT からのレスポンス
pub async fn request_reply_message(
    reply_messages: &[ReplyMessage],
    model: ChatGPTEngine,
) -> anyhow::Result<MessageCompletionResult> {
    let client = init_client(OPENAI_API_KEY.as_str(), Some(model))?;

    let mut history = reply_messages
        .iter()
        .map(|reply| ChatMessage {
            content: reply.content.clone(),
            role: match reply.role {
                ReplyRole::Ichiyo => Role::Assistant,
                ReplyRole::User => Role::User,
            },
        })
        .collect::<Vec<ChatMessage>>();

    history.insert(
        0,
        ChatMessage {
            role: Role::System,
            content: SYSTEM_CONTEXT.to_string(),
        },
    );

    let response = timeout(TIMEOUT_DURATION, client.send_history(&history))
        .await
        .context("タイムアウトしました, もう一度お試しください.")??;

    let result = MessageCompletionResult {
        message: response.message().content.clone(),
        input_token: response.usage.prompt_tokens,
        output_token: response.usage.completion_tokens,
    };

    Ok(result)
}
