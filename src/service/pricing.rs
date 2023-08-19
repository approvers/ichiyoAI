use chatgpt::prelude::ChatGPTEngine;

const GPT3_5_PRICE_PER_INPUT_1K_TOKEN_USD: f32 = 0.0015;
const GPT3_5_PRICE_PER_OUTPUT_1K_TOKEN_USD: f32 = 0.002;
const GPT4_PRICE_PER_INPUT_1K_TOKEN_USD: f32 = 0.03;
const GPT4_PRICE_PER_OUTPUT_1K_TOKEN_USD: f32 = 0.06;

// クレジットカード会社の手数料も含めて、多めに設定
const EXCHANGE_RATE_USD_PER_JPY: f32 = 150.0;

pub fn usage_pricing(input_token: u32, output_token: u32, model: ChatGPTEngine) -> f32 {
    let input_token_per_usd = match model {
        ChatGPTEngine::Gpt35Turbo => GPT3_5_PRICE_PER_INPUT_1K_TOKEN_USD,
        ChatGPTEngine::Gpt4 => GPT4_PRICE_PER_INPUT_1K_TOKEN_USD,
        _ => panic!("Invalid model: {:?}", model),
    } * 0.001;
    let output_token_per_usd = match model {
        ChatGPTEngine::Gpt35Turbo => GPT3_5_PRICE_PER_OUTPUT_1K_TOKEN_USD,
        ChatGPTEngine::Gpt4 => GPT4_PRICE_PER_OUTPUT_1K_TOKEN_USD,
        _ => panic!("Invalid model: {:?}", model),
    } * 0.001;

    let input_usd = input_token as f32 * input_token_per_usd;
    let output_usd = output_token as f32 * output_token_per_usd;

    let usd = input_usd + output_usd;
    let jpy = usd * EXCHANGE_RATE_USD_PER_JPY;

    jpy
}
