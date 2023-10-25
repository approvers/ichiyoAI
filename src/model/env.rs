use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IchiyoAiEnv {
    pub discord_api_token: String,
    pub openai_api_key: String,
    pub cargo_pkg_version: String,
    pub guild_id: u64,
    pub taxpayer_role_id: u64,
}

pub static ICHIYOAI_ENV: OnceCell<IchiyoAiEnv> = OnceCell::new();