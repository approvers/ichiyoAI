use std::env;

use chatgpt::config::ModelConfigurationBuilder;
use chatgpt::prelude::{ChatGPT, ChatGPTEngine};
use chatgpt::types::CompletionResponse;
use once_cell::sync::Lazy;

static CHATGPT_API_TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("CHATGPT_API_TOKEN").expect("Expected a token in the environment (CHATGPT_API_TOKEN)")
});

pub async fn chat_completion(
    message: &str,
    model: Option<ChatGPTEngine>,
) -> chatgpt::Result<CompletionResponse> {
    let model = model.unwrap_or(ChatGPTEngine::Gpt35Turbo);
    let client = ChatGPT::new_with_config(
        &*CHATGPT_API_TOKEN,
        ModelConfigurationBuilder::default()
            .engine(model)
            .build()
            .unwrap(),
    )?;

    client.send_message(message).await
}
