use crate::model::chatgpt::{RequestMessageModel, ResponseCompletionResultModel};
use anyhow::Context;
use async_openai::config::OpenAIConfig;
use async_openai::types::CreateChatCompletionRequestArgs;
use async_openai::Client;
use std::time::Duration;
use tokio::time::timeout;
use tracing::info;
use tracing::log::debug;

pub static SYSTEM_CONTEXT: &str = "回答時は以下のルールに従うこと.\n- 1900文字以内に収めること。";
static TIMEOUT_DURATION: Duration = Duration::from_secs(180);

async fn create_chatgpt_client() -> anyhow::Result<Client<OpenAIConfig>> {
    Ok(Client::new())
}

pub async fn request_chatgpt_message(
    request: RequestMessageModel,
) -> anyhow::Result<ResponseCompletionResultModel> {
    let client = create_chatgpt_client().await?;

    let client_request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model(request.model)
        .messages(request.replies)
        .build()?;

    let response = timeout(TIMEOUT_DURATION, client.chat().create(client_request))
        .await
        .context("Timeout. Please try again.")??;

    let choice = response
        .choices
        .get(0)
        .context("No response message found.")?;
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

    Ok(ResponseCompletionResultModel::builder()
        .response_message(choice.message.content.clone().unwrap_or_default())
        .input_token(input_token)
        .output_token(output_token)
        .total_token(total_token)
        .build())
}
