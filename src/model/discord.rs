use serenity::http::Http;
use serenity::model::prelude::{ChannelId, Message, MessageId};
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct DiscordMessageModel {
    pub content: String,
    pub message_channel_id: ChannelId,
    pub reference_message_id: Option<MessageId>,
}

#[derive(TypedBuilder)]
pub struct DiscordReplyMessageModel {
    pub http: Arc<Http>,
    pub target_message: Message,
    pub formatted_result: String,
}
