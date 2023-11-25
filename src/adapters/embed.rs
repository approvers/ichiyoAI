use crate::model::dall_e::DaLLEResponseModel;
use crate::model::embed::{EmbedMessage, EmbedMessageFooter, EmbedMessageImage};
use serenity::{builder::CreateEmbed, utils::Colour};
use tracing::info;

fn convert_embed(
    EmbedMessage {
        title,
        author,
        description,
        fields,
        footer,
        color,
        url,
        image,
        thumbnail,
        timestamp,
    }: EmbedMessage,
) -> CreateEmbed {
    let mut create_embed = CreateEmbed::default();

    if let Some(title) = title {
        create_embed.title(title);
    }

    if let Some(author) = author {
        create_embed.author(|a| {
            a.name(author.name);
            if let Some(url) = author.url {
                a.url(url);
            }
            if let Some(icon_url) = author.icon_url {
                a.icon_url(icon_url);
            };
            a
        });
    }

    if let Some(description) = description {
        create_embed.description(description);
    }

    if let Some(fields) = fields {
        fields.into_iter().for_each(|x| {
            create_embed.field(x.name, x.value, x.inline.unwrap_or(false));
        })
    }

    if let Some(footer) = footer {
        create_embed.footer(|f| {
            f.text(footer.text);
            if let Some(icon_url) = footer.icon_url {
                f.icon_url(icon_url);
            };
            f
        });
    }

    if let Some(color) = color {
        create_embed.color(Colour(color));
    }

    if let Some(url) = url {
        create_embed.url(url);
    }

    if let Some(image) = image {
        if let Some(image_url) = image.url {
            create_embed.image(image_url);
        }
    }

    if let Some(thumbnail) = thumbnail {
        create_embed.thumbnail(thumbnail.url);
    }

    if let Some(timestamp) = timestamp {
        create_embed.timestamp(timestamp);
    }

    create_embed
}

pub fn build_davinci_embed(response: DaLLEResponseModel) -> anyhow::Result<CreateEmbed> {
    // TODO: remove when stable image generation
    let footer = EmbedMessageFooter::builder()
        .text(
            "Image Generation機能は現在ベータ版です. 予期せぬ不具合が発生する可能性があります."
                .to_string(),
        )
        .build();
    let image = EmbedMessageImage::builder()
        .url(Some(response.res_image_url))
        .build();

    let embed = EmbedMessage::builder()
        .title(Some("生成結果".to_string()))
        .description(Some(response.prompt))
        .image(Some(image))
        .footer(Some(footer))
        .build();
    Ok(convert_embed(embed))
}
