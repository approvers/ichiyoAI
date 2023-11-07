use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Serialize, Deserialize, Debug, EnumString, Display)]
pub enum LogEnvironment {
    #[strum(serialize = "production")]
    Production,

    #[strum(serialize = "development")]
    Development,
}

impl Default for LogEnvironment {
    fn default() -> Self {
        LogEnvironment::Production
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IchiyoAiEnv {
    pub discord_api_token: String,
    pub openai_api_key: String,
    pub guild_id: u64,
    pub taxpayer_role_id: u64,
    #[serde(default)]
    pub log_environment: LogEnvironment,
}

pub static ICHIYOAI_ENV: OnceCell<IchiyoAiEnv> = OnceCell::new();
