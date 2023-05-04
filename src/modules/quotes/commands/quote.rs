use crate::modules::quotes::queries::*;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Saves a quote of a user.
#[poise::command(slash_command, prefix_command)]
pub async fn quote(
    ctx: Context<'_>,
    #[description = "User to quote"] user: serenity::User,
    #[description = "The quote"] quote: String,
) -> Result<(), Error> {
    log::info!("Received quote command from {}", ctx.author().name);
    let u = &user;

    log::debug!("{:#?}", u);

    let (guild_id, quoted_by, quote_string, author): (String, String, String, String) = (
        ctx.guild_id().unwrap().to_string(),
        ctx.author().name.clone(),
        quote,
        user.name.clone(),
    );

    if let Ok(_) = create_quote(guild_id, quoted_by, quote_string, author).await {
        ctx.say("Quote saved.").await?;
    } else {
        ctx.say("Failed to save quote.").await?;
    }
    Ok(())
}
