use serenity::async_trait;
use serenity::model::prelude::Ready;
use serenity::prelude::{Context, EventHandler};

pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} (ID: {}) にログインしました。", ready.user.name, ready.user.id);
    }

}
