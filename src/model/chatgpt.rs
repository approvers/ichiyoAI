use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Serialize, Deserialize, Debug)]
pub struct ChatGPTResponseModel {
    pub model: String,
    pub res: String,
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

const SCALE: f32 = 10_000_000.0;
const EXCHANGE_RATE: f32 = 150.0;

// https://openai.com/pricing
const GPT3_5_TURBO_JPY_PER_INPUT_TOKEN: u32 = (0.0010 * EXCHANGE_RATE * SCALE / 1000.0) as u32;
const GPT3_5_TURBO_JPY_PER_OUTPUT_TOKEN: u32 = (0.0020 * EXCHANGE_RATE * SCALE / 1000.0) as u32;
const GPT4_TURBO_JPY_PER_INPUT_TOKEN: u32 = (0.01 * EXCHANGE_RATE * SCALE / 1000.0) as u32;
const GPT4_TURBO_JPY_PER_OUTPUT_TOKEN: u32 = (0.03 * EXCHANGE_RATE * SCALE / 1000.0) as u32;

pub fn usage_pricing(prompt_tokens: u32, completion_tokens: u32, model: &str) -> f32 {
    let (input_rate, output_rate) = match model {
        "gpt-3.5-turbo-1106" => (
            GPT3_5_TURBO_JPY_PER_INPUT_TOKEN,
            GPT3_5_TURBO_JPY_PER_OUTPUT_TOKEN,
        ),
        "gpt-4-1106-preview" => (
            GPT4_TURBO_JPY_PER_INPUT_TOKEN,
            GPT4_TURBO_JPY_PER_OUTPUT_TOKEN,
        ),
        _ => panic!("Invalid model: {:?}", model),
    };

    (input_rate * prompt_tokens + output_rate * completion_tokens) as f32 / SCALE
}
