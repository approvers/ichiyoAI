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
