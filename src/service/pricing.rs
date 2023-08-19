use chatgpt::prelude::ChatGPTEngine;

const INPUT_TOKEN_PER_USD_GPT3_5: f32 = 0.0015;
const OUTPUT_TOKEN_PER_USD_GPT3_5: f32 = 0.002;
const INPUT_TOKEN_PER_USD_GPT4: f32 = 0.03;
const OUTPUT_TOKEN_PER_USD_GPT4: f32 = 0.06;
const EXCHANGE_RATE_USD_PER_JPY: f32 = 142.0;

pub fn usage_pricing(input_token: u32, output_token: u32, model: ChatGPTEngine) -> u32 {
    let input_token_per_usd = match model {
        ChatGPTEngine::Gpt35Turbo => INPUT_TOKEN_PER_USD_GPT3_5,
        ChatGPTEngine::Gpt4 => INPUT_TOKEN_PER_USD_GPT4,
        _ => panic!("Invalid model: {:?}", model),
    };
    let output_token_per_usd = match model {
        ChatGPTEngine::Gpt35Turbo => OUTPUT_TOKEN_PER_USD_GPT3_5,
        ChatGPTEngine::Gpt4 => OUTPUT_TOKEN_PER_USD_GPT4,
        _ => panic!("Invalid model: {:?}", model),
    };

    let input_usd = input_token as f32 * input_token_per_usd;
    let output_usd = output_token as f32 * output_token_per_usd;

    let usd = input_usd + output_usd;
    let jpy = usd * EXCHANGE_RATE_USD_PER_JPY;

    jpy as u32
}
