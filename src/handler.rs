use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::info;

use crate::modules::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        // Register module commands.
        development::register_commands(ctx).await;

        info!("Initialized commands!")
    }
}
