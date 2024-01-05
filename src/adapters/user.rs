use anyhow::Context as _;
use serenity::{client::Context, model::user::User};

pub async fn is_sponsor(ctx: &Context, user: &User) -> anyhow::Result<bool> {
    let envs = crate::model::env::envs();

    let guild_id = envs.guild_id;
    let role_id = envs.sponsor_role_id;

    user.has_role(ctx, guild_id, role_id)
        .await
        .context("Failed to get sponsor role")
}
