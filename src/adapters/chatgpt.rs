use crate::{
    adapters::{message::start_typing, TIMEOUT_DURATION},
    model::chatgpt::ChatGPTResponseModel,
};
use anyhow::Context as _;
use async_openai::{
    types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs},
    Client,
};
use serenity::{all::ChannelId, client::Context};
use tokio::time::timeout;
use tracing::info;

pub async fn request_chatgpt_response(
    ctx: &Context,
    channel_id: ChannelId,
    prompts: Vec<ChatCompletionRequestMessage>,
    is_gpt4: bool,
) -> anyhow::Result<ChatGPTResponseModel> {
    let typing = start_typing(ctx.http.clone(), channel_id);

    let client = Client::new();
    let model = match is_gpt4 {
        // notes: gpt-4-1106-preview is preview model
        true => "gpt-4-1106-preview",
        false => "gpt-3.5-turbo-1106",
    };

    info!("Request: {:?}", prompts);

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(prompts)
        .build()?;

    let response = timeout(TIMEOUT_DURATION, client.chat().create(request))
        .await
        .context("Operation timed out")??;

    info!("Response: {:?}", response.choices);
    let choice = response
        .choices
        .get(0)
        .context("No response message found")?;
    let (prompt_tokens, completion_tokens, total_tokens) = response
        .usage
        .map(|usage| {
            (
                usage.prompt_tokens,
                usage.completion_tokens,
                usage.total_tokens,
            )
        })
        .unwrap_or_default();

    typing.stop();
    Ok(ChatGPTResponseModel::builder()
        .model(model.to_string())
        .res(choice.message.content.clone().unwrap_or_default())
        .prompt_tokens(prompt_tokens)
        .completion_tokens(completion_tokens)
        .total_tokens(total_tokens)
        .build())
}
