use crate::{Context, Error};

/// Replies with "Pong."
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let response = "Pong.";
    ctx.say(response).await?;
    Ok(())
}
