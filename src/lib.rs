extern crate alloc;
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

pub trait Metadata {
    fn tokens(&self) -> usize;
    fn price_yen(&self) -> f64;
    fn by(&self) -> &str;
}

pub trait Completion {
    type Metadata: Metadata;

    fn next<I: Send + Sync>(
        &self,
        messages: &[Message<I>],
    ) -> impl Future<Output = anyhow::Result<(Message<()>, Self::Metadata)>> + Send + Sync;
}

mod gemini;
mod openai;

pub use gemini::Gemini;
pub type OpenAiGPT4Turbo = openai::OpenAi<openai::GPT4Turbo>;
pub type OpenAiGPT35Turbo = openai::OpenAi<openai::GPT35Turbo>;

pub trait Image {
    fn create(
        &self,
        prompt: impl AsRef<str> + Send + Sync,
    ) -> impl Future<Output = anyhow::Result<GeneratedImage>> + Send + Sync;
}

pub struct GeneratedImage {
    pub image: Vec<u8>,
    pub prompt: String,
    pub ext: ImageExt,
}

#[non_exhaustive]
pub enum ImageExt {
    Png,
}

impl ImageExt {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Png => "PNG",
        }
    }
}

mod dalle;

pub type OpenAiDallE2 = dalle::OpenAi<dalle::DallE2>;
pub type OpenAiDallE3 = dalle::OpenAi<dalle::DallE3>;
