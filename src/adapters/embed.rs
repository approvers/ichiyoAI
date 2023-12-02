use crate::model::dall_e::DaLLEResponseModel;
use anyhow::Ok;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};

pub fn build_davinci_embed(response: DaLLEResponseModel) -> anyhow::Result<CreateEmbed> {
    let footer = CreateEmbedFooter::new(format!("Model: {}", response.model));
    let embed = CreateEmbed::new()
        .title("生成結果")
        .url(response.res_image_url.clone())
        .description(response.prompt)
        .image(response.res_image_url)
        .footer(footer);

    Ok(embed)
}
