use poise::serenity_prelude::CreateEmbed;

use crate::{Context, Error};

fn create_embed<'a>(embed: &'a mut CreateEmbed, body: &'a str) -> &'a mut CreateEmbed {
    embed
        .color(0xff0000)
        .description(body)
        .field("Ping", "Pong", false)
}

/// Replies with "Pong."
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    log::info!("Received ping command from {}", ctx.author().name);

    ctx.send(|r| r.embed(|e| create_embed(e, "This is the body")))
        .await
        .unwrap();

    Ok(())
}
