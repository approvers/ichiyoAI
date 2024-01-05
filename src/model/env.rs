use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Serialize, Deserialize, Debug)]
pub struct IchiyoAiEnv {
    pub discord_api_token: String,
    pub openai_api_key: String,
    pub google_ai_api_key: String,
    pub guild_id: u64,
    pub sponsor_role_id: u64,
}

pub fn envs() -> &'static IchiyoAiEnv {
    static CACHE: OnceLock<IchiyoAiEnv> = OnceLock::new();

    CACHE.get_or_init(|| envy::from_env().expect("Failed to load enviroment variables"))
}
