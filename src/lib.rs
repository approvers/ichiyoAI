extern crate alloc;
use core::future::Future;

pub enum Message {
    User { content: String },
    Model { content: String },
}

impl Message {
    pub fn content(&self) -> &str {
        match self {
            Self::User { content, .. } => content,
            Self::Model { content, .. } => content,
        }
    }
}

pub struct Metadata {
    pub tokens: usize,
    pub price_yen: f64,
    pub by: &'static str,
}

pub trait Completion {
    fn next(
        &self,
        messages: &[Message],
    ) -> impl Future<Output = anyhow::Result<(Message, Metadata)>> + Send + Sync;
}

mod google;
mod openai;

pub type GoogleGeminiPro = google::Google<google::GeminiPro>;
pub type OpenAiGPT4Turbo = openai::OpenAi<openai::GPT4Turbo>;
pub type OpenAiGPT35Turbo = openai::OpenAi<openai::GPT35Turbo>;

pub trait Image {
    type Metadata;

    fn create(
        &self,
        prompt: impl AsRef<str> + Send + Sync,
    ) -> impl Future<Output = anyhow::Result<(GeneratedImage, Self::Metadata)>> + Send + Sync;
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
            Self::Png => "png",
        }
    }
}

// FIXME: needless `pub`
pub mod dalle;

pub type OpenAiDallE2 = dalle::OpenAi<dalle::DallE2>;
pub type OpenAiDallE3 = dalle::OpenAi<dalle::DallE3>;
