use crate::{Context, Error};

/// Replies with "Pong."
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    log::info!("Received ping command from {}", ctx.author().name);
    ctx.say("Pong.").await?;
    Ok(())
}
