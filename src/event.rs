use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::gateway::Ready,
    model::{channel::Message, gateway::Activity},
};
use tracing::info;

pub struct EvHandler;

#[async_trait]
impl EventHandler for EvHandler {
    async fn message(&self, ctx: Context, message: Message) {}

    async fn ready(&self, ctx: Context, self_bot: Ready) {
        info!("Starting...");

        let version = env!("CARGO_PKG_VERSION");
        ctx.set_activity(Activity::playing(&format!("v{}", version)))
            .await;

        info!("Running ichiyoAI v{}", version);
        info!(
            "Connected!: {name}(Id:{id})",
            name = self_bot.user.name,
            id = self_bot.user.id
        )
    }
}
