use std::time::Duration;

pub mod message;
pub mod user;

// DaLL-E & ChatGPT で使用するタイムアウト時間の定数
pub static TIMEOUT_DURATION: Duration = Duration::from_secs(180);
