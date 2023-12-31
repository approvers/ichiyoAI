use core::future::Future;

pub enum Message<I> {
    User { id: I, content: String },
    Model { id: I, content: String },
}

impl<I> Message<I> {
    pub fn id(&self) -> &I {
        match self {
            Self::User { id, .. } => id,
            Self::Model { id, .. } => id,
        }
    }

    pub fn content(&self) -> &str {
        match self {
            Self::User { content, .. } => content,
            Self::Model { content, .. } => content,
        }
    }
}

pub trait Completion {
    type Metadata;

    fn next<I: Send + Sync>(
        &self,
        messages: &[Message<I>],
    ) -> impl Future<Output = anyhow::Result<(Message<()>, Self::Metadata)>> + Send + Sync;
}

mod gemini;
mod gpt4;
