#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplyRole {
    Ichiyo,
    User,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplyMessage {
    pub role: ReplyRole,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageCompletionResult {
    pub message: String,
    pub input_token: u32,
    pub output_token: u32,
    pub total_token: u32,
}
