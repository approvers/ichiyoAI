use anyhow::Context as _;
use async_openai::{
    types::{CreateImageRequestArgs, Image, ImageModel, ImageSize, ResponseFormat},
    Client,
};
use serenity::{all::ChannelId, client::Context};
use tokio::time::timeout;

use crate::model::dall_e::DaLLEResponseModel;

use super::{message::start_typing, TIMEOUT_DURATION};

pub async fn generate_dall_e_image(
    ctx: &Context,
    channel_id: ChannelId,
    prompt: String,
    is_dell4: bool,
) -> anyhow::Result<DaLLEResponseModel> {
    let typing = start_typing(ctx.http.clone(), channel_id);

    let client = Client::new();
    let mut image_url = String::new();

    let model = match is_dell4 {
        true => ImageModel::DallE3,
        false => ImageModel::DallE2,
    };

    let request = CreateImageRequestArgs::default()
        .prompt(prompt.clone())
        .model(model.clone())
        .n(1)
        .response_format(ResponseFormat::Url)
        .size(ImageSize::S1024x1024)
        .user("ichiyoai-image-generation")
        .build()?;

    let response = timeout(TIMEOUT_DURATION, client.images().create(request))
        .await
        .context("Operation timed out")??;

    response.data.iter().for_each(|x| match &**x {
        Image::Url { url, .. } => {
            let _ = url.replace('!', "");
            image_url = url.to_string();
        }
        _ => {}
    });

    typing.stop();
    Ok(DaLLEResponseModel::builder()
        .model(model.to_string())
        .prompt(prompt)
        .res_image_url(image_url)
        .build())
}
