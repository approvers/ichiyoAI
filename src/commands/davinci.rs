use serenity::builder::{
    CreateAllowedMentions, CreateAttachment, CreateEmbed, CreateEmbedFooter, CreateMessage,
};
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use tracing::error;

use crate::adapters::message::start_typing;
use crate::adapters::user::is_sponsor;
use crate::model::env::ICHIYOAI_ENV;

#[command]
#[aliases(image)]
async fn davinci(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let prompt = args.raw_quoted().collect::<String>();

    let is_dalle4 = match is_sponsor(ctx, &msg.author).await {
        Ok(is_gpt4) => is_gpt4,
        Err(why) => {
            error!("Failed to check sponsor: {}", why);
            false
        }
    };

    let token = ICHIYOAI_ENV
        .get()
        .expect("Failed to get openai api key from env")
        .openai_api_key
        .as_str();

    let typing = start_typing(ctx.http.clone(), msg.channel_id);

    let result = match is_dalle4 {
        true => {
            use anyhow::Context as _;
            use ichiyo_ai::Image as _;

            let engine = ichiyo_ai::OpenAiDallE3::new(token);
            tokio::time::timeout(crate::adapters::TIMEOUT_DURATION, engine.create(prompt))
                .await
                .context("Operation timed out")
        }
        false => {
            use anyhow::Context as _;
            use ichiyo_ai::Image as _;

            let engine = ichiyo_ai::OpenAiDallE2::new(token);
            tokio::time::timeout(crate::adapters::TIMEOUT_DURATION, engine.create(prompt))
                .await
                .context("Operation timed out")
        }
    };

    typing.stop();

    let (response, metadata) = match result {
        Ok(Ok(response)) => response,
        Ok(Err(why)) | Err(why) => {
            let _ = msg
                .reply_ping(&ctx, format!("An error has occurred: {}", why))
                .await;
            error!("Failed to generate dall-e image: {}", why);
            return Ok(());
        }
    };

    let filename = format!("image.{}", response.ext.as_str());
    let image = CreateAttachment::bytes(response.image, &filename);

    let embed = CreateEmbed::new()
        .title("生成結果")
        .attachment(filename)
        .description(response.prompt)
        .footer(CreateEmbedFooter::new(format!("Model: {}", metadata.model)));

    let message = CreateMessage::default()
        .reference_message(msg)
        .allowed_mentions(CreateAllowedMentions::default().replied_user(true))
        .embed(embed);

    msg.channel_id
        .send_files(&ctx.http, [image], message)
        .await
        .expect("Failed to send message.");

    Ok(())
}
