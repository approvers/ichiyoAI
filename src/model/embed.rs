use serenity::model::Timestamp;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct EmbedMessageAuthor {
    pub name: String,

    #[builder(default)]
    pub icon_url: Option<String>,

    #[builder(default)]
    pub url: Option<String>,
}

#[derive(TypedBuilder)]
pub struct EmbedMessageField {
    pub name: String,

    pub value: String,

    #[builder(default)]
    pub inline: Option<bool>,
}

#[derive(TypedBuilder)]
pub struct EmbedMessageFooter {
    pub text: String,

    #[builder(default)]
    pub icon_url: Option<String>,
}

#[derive(TypedBuilder)]
pub struct EmbedMessageImage {
    #[builder(default)]
    pub url: Option<String>,

    #[builder(default)]
    pub proxy_url: Option<String>,

    #[builder(default)]
    pub height: Option<String>,

    #[builder(default)]
    pub width: Option<String>,
}

#[derive(TypedBuilder)]
pub struct EmbedMessageThumbnail {
    pub url: String,
}

#[derive(TypedBuilder)]
pub struct EmbedMessage {
    #[builder(default)]
    pub author: Option<EmbedMessageAuthor>,

    #[builder(default)]
    pub title: Option<String>,

    #[builder(default)]
    pub description: Option<String>,

    #[builder(default)]
    pub fields: Option<Vec<EmbedMessageField>>,

    #[builder(default)]
    pub footer: Option<EmbedMessageFooter>,

    #[builder(default)]
    pub color: Option<u32>,

    #[builder(default)]
    pub url: Option<String>,

    #[builder(default)]
    pub image: Option<EmbedMessageImage>,

    #[builder(default)]
    pub thumbnail: Option<EmbedMessageThumbnail>,

    #[builder(default)]
    pub timestamp: Option<Timestamp>,
}
