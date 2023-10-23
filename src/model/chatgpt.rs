use crate::model::discord::DiscordMessageModel;
use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct RequestMessageModel {
    pub replies: Vec<ChatCompletionRequestMessage>,
    pub model: String,
}

#[derive(TypedBuilder)]
pub struct ResponseCompletionResultModel {
    pub response_message: String,
    pub input_token: u32,
    pub output_token: u32,
    pub total_token: u32,
}

// 桁落ちを防ぐため、10,000,000倍して計算する
const SCALE: f32 = 10_000_000.0;

// クレジットカード会社の手数料も含めて、多めに設定
const EXCHANGE_RATE: f32 = 150.0;

// https://openai.com/pricing
const GPT3_5_JPY_PER_INPUT_TOKEN: u32 = (0.0015 * EXCHANGE_RATE * SCALE / 1000.0) as u32;
const GPT3_5_JPY_PER_OUTPUT_TOKEN: u32 = (0.002 * EXCHANGE_RATE * SCALE / 1000.0) as u32;
const GPT4_JPY_PER_INPUT_TOKEN: u32 = (0.03 * EXCHANGE_RATE * SCALE / 1000.0) as u32;
const GPT4_JPY_PER_OUTPUT_TOKEN: u32 = (0.06 * EXCHANGE_RATE * SCALE / 1000.0) as u32;

pub fn usage_pricing(input_token: u32, output_token: u32, model: &str) -> f32 {
    let (input_rate, output_rate) = match model {
        "gpt-3.5-turbo" => (GPT3_5_JPY_PER_INPUT_TOKEN, GPT3_5_JPY_PER_OUTPUT_TOKEN),
        "gpt-4" => (GPT4_JPY_PER_INPUT_TOKEN, GPT4_JPY_PER_OUTPUT_TOKEN),
        _ => panic!("Invalid model: {:?}", model),
    };

    (input_rate * input_token + output_rate * output_token) as f32 / SCALE
}
