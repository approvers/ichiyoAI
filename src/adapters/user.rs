use anyhow::Context as _;
use serenity::{
    client::Context,
    model::{
        id::{GuildId, RoleId},
        user::User,
    },
};

use crate::model::env::ICHIYOAI_ENV;

pub async fn is_sponsor(ctx: &Context, msg_author: &User) -> anyhow::Result<bool> {
    msg_author
        .has_role(
            &ctx,
            GuildId::new(ICHIYOAI_ENV.get().unwrap().guild_id),
            RoleId::new(ICHIYOAI_ENV.get().unwrap().sponsor_role_id),
        )
        .await
        .context("Failed to get sponsor role")
}
