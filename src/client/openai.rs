use crate::model::{MessageCompletionResult, ReplyMessage, ReplyRole};
use anyhow::{Context, Ok};
use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use std::time::Duration;
use tokio::time::timeout;

static TIMEOUT_DURATION: Duration = Duration::from_secs(180);

// 会話モード・返信モード で使用するシステムコンテキスト。膨大なレスポンスにならないように抑える目的に使用する。
// レスポンスの後にメタ情報（利用料金表示など）を含めるため、100字分の余裕を設けている。
static SYSTEM_CONTEXT: &str = "回答時は以下のルールに従うこと.\n- 1900文字以内に収めること。\n- なるべく簡潔に言うこと。\n- 一般的に知られている単語は説明しない。";

/// ChatGPT に対してメッセージを送信し、レスポンスをリクエストします。
///
/// ### 引数
/// * `request_message` -- ChatGPT に送信するメッセージ。[ReplyMessages] を実装しておく必要がある。
/// * `model` --
///         使用する ChatGPT のモデルを使用する。使用できるモデルは [&str] で定義されている物のみ。
///         指定しない場合([None])は [&str::Gpt35Turbo] が使用される。
/// ### 返り値
/// [String]: ChatGPT からのレスポンス
///
/// ### エラー
/// 下記条件でエラーが報告されます。
/// * ChatGPT とのやり取りに失敗する
/// * 2000文字を超過する
pub async fn request_message(
    request_message: &[ReplyMessage],
    model: &str,
) -> anyhow::Result<MessageCompletionResult> {
    let client = Client::new();

    let mut messages = vec![ChatCompletionRequestMessageArgs::default()
        .role(Role::System)
        .content(SYSTEM_CONTEXT)
        .build()?];
    let history = request_message
        .iter()
        .map(|reply| {
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(reply.content.clone())
                .build()
        })
        .collect::<Result<Vec<_>, _>>()?;
    messages.extend(history);

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages)
        .build()?;

    let response = timeout(TIMEOUT_DURATION, client.chat().create(request))
        .await
        .context("タイムアウトしました, もう一度お試しください.")??;

    let choice = response
        .choices
        .get(0)
        .context("response message not found")?;
    let (input_token, output_token, total_token) = response
        .usage
        .map(|usage| {
            (
                usage.prompt_tokens,
                usage.completion_tokens,
                usage.total_tokens,
            )
        })
        .unwrap_or_default();

    let result = MessageCompletionResult {
        message: choice.message.content.clone().unwrap_or_default(),
        input_token,
        output_token,
        total_token,
    };

    Ok(result)
}

/// ChatGPT に対して一連の会話コンテキストを送信し、レスポンスをリクエストします。
///
/// ### 引数
/// * `reply_messages` -- ChatGPT　に送信する会話コンテキスト。[ReplyMessages] を実装しておく必要がある。
/// * `model` --
///         使用する ChatGPT のモデルを使用する。使用できるモデルは [&str] で定義されている物のみ。
///         指定しない場合([None])は [&str::Gpt35Turbo] が使用される。
///
/// ### 返り値
/// [String]: ChatGPT からのレスポンス
pub async fn request_reply_message(
    reply_messages: &[ReplyMessage],
    model: &str,
) -> anyhow::Result<MessageCompletionResult> {
    let client = Client::new();

    let mut messages = vec![ChatCompletionRequestMessageArgs::default()
        .role(Role::System)
        .content(SYSTEM_CONTEXT)
        .build()?];
    let history = reply_messages
        .iter()
        .map(|reply| {
            ChatCompletionRequestMessageArgs::default()
                .role(match reply.role {
                    ReplyRole::Ichiyo => Role::Assistant,
                    ReplyRole::User => Role::User,
                })
                .content(reply.content.clone())
                .build()
        })
        .collect::<Result<Vec<_>, _>>()?;
    messages.extend(history);

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages)
        .build()?;

    let response = timeout(TIMEOUT_DURATION, client.chat().create(request))
        .await
        .context("タイムアウトしました, もう一度お試しください.")??;

    let choice = response
        .choices
        .get(0)
        .context("response message not found")?;
    let (input_token, output_token, total_token) = response
        .usage
        .map(|usage| {
            (
                usage.prompt_tokens,
                usage.completion_tokens,
                usage.total_tokens,
            )
        })
        .unwrap_or_default();

    let result = MessageCompletionResult {
        message: choice.message.content.clone().unwrap_or_default(),
        input_token,
        output_token,
        total_token,
    };

    Ok(result)
}
