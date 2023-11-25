use async_openai::{
    types::{CreateImageRequestArgs, Image, ImageModel, ImageSize, ResponseFormat},
    Client,
};
use serenity::client::Context;

use crate::model::dall_e::DaLLEResponseModel;

use super::message::start_typing;

pub async fn generate_dall_e_image(
    ctx: &Context,
    channel_id: u64,
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

    let response = client.images().create(request).await?;

    response.data.iter().for_each(|x| match &**x {
        Image::Url { url, .. } => {
            let _ = url.replace("!", "");
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
