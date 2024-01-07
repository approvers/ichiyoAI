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

    pub fn is_user(&self) -> bool {
        match self {
            Self::User { .. } => true,
            Self::Model { .. } => false,
        }
    }

    pub fn is_model(&self) -> bool {
        match self {
            Self::User { .. } => false,
            Self::Model { .. } => true,
        }
    }
}

pub struct CMetadata {
    pub tokens: usize,
    pub price_yen: f64,
    pub by: &'static str,
}

pub trait Completion {
    fn next(
        &self,
        messages: &[Message],
    ) -> impl Future<Output = Result<(Message, CMetadata), alloc::borrow::Cow<'static, str>>> + Send + Sync;
}

mod google;
mod openai;

pub type GoogleGeminiPro = google::Google<google::GeminiPro>;
pub type OpenAiGPT4Turbo = openai::OpenAi<openai::GPT4Turbo>;
pub type OpenAiGPT35Turbo = openai::OpenAi<openai::GPT35Turbo>;

pub trait Generation {
    fn create(
        &self,
        prompt: impl AsRef<str> + Send + Sync,
    ) -> impl Future<Output = Result<(Image, IMetadata), alloc::borrow::Cow<'static, str>>> + Send + Sync;
}

pub struct Image {
    pub raw: Vec<u8>,
    pub prompt: String,
    pub ext: ImageExt,
}

pub struct IMetadata {
    pub model: &'static str,
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

mod dalle;

pub type OpenAiDallE2 = dalle::OpenAi<dalle::DallE2>;
pub type OpenAiDallE3 = dalle::OpenAi<dalle::DallE3>;
