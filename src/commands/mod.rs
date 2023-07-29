use crate::commands::direct::command_direct;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub mod direct;

#[command]
#[aliases(roleplay)]
async fn direct(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(why) = command_direct(ctx, msg, args).await {
        let _ = msg
            .reply_ping(&ctx, &format!("エラーが発生しました。\n```{}\n```", why))
            .await;
        eprintln!("{:?}", why);
    }

    Ok(())
}
