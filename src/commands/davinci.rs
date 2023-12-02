use serenity::builder::{CreateAllowedMentions, CreateMessage};
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use tracing::error;

use crate::adapters::dall_e::generate_dall_e_image;
use crate::adapters::embed::build_davinci_embed;
use crate::adapters::user::is_sponsor;

#[command]
#[aliases(image)]
async fn davinci(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let prompt = args.raw_quoted().collect::<String>();

    let is_dalle4 = match is_sponsor(ctx, msg.clone().author).await {
        Ok(is_gpt4) => is_gpt4,
        Err(why) => {
            error!("Failed to check sponsor: {}", why);
            false
        }
    };

    let response = match generate_dall_e_image(ctx, msg.channel_id, prompt, is_dalle4).await {
        Ok(response) => response,
        Err(why) => {
            let _ = msg
                .reply_ping(&ctx, format!("An error has occurred: {}", why))
                .await;
            error!("Failed to generate dall-e image: {}", why);
            return Ok(());
        }
    };

    let embed = build_davinci_embed(response).expect("Failed to build embed.");
    let message = CreateMessage::default()
        .reference_message(msg)
        .allowed_mentions(CreateAllowedMentions::default().replied_user(true))
        .embed(embed);

    msg.channel_id
        .send_message(&ctx.http, message)
        .await
        .expect("Failed to send message.");

    Ok(())
}
