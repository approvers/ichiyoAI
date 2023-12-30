use core::future::Future;

pub enum Message<I> {
    User { id: I, content: String },
    Model { id: I, content: String },
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
