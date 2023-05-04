use crate::db::entity::quotes::Model;
use crate::modules::quotes::queries::*;
use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, CreateEmbed};
use rand::seq::SliceRandom;

const LOCALES: [&str; 5] = ["en", "fr", "ja", "de", "zh-CN"];

fn create_embed<'a>(embed: &'a mut CreateEmbed, quote: Model) -> &'a mut CreateEmbed {
    embed
        .color(0xe43a25)
        .title("Archives of Wisdom")
        .description(format!(
            r#""{}"
        
        - {}"#,
            quote.quote_string, quote.author
        ))
        .field("Date Added", quote.created_at.unwrap(), true)
        .field("Added By", quote.quoted_by, true)
}

/// Save a quote.
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

/// Read off a quote.
#[poise::command(slash_command, prefix_command)]
pub async fn inspireme(
    ctx: Context<'_>,
    #[description = "Filter by user. (default: random)"] author: Option<serenity::User>,
    #[description = "Specify a locale. (default: random)."] locale: Option<String>,
    #[description = "Allow pulling quotes from other servers. (default: false)."] global: Option<
        bool,
    >,
) -> Result<(), Error> {
    log::info!("Received inspireme command from {}", ctx.author().name);

    let locale = locale.unwrap_or_else(|| {
        LOCALES
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
            .to_string()
    });
    let global = global.unwrap_or_else(|| false);
    let quote;

    if let Some(user) = author {
        // Get a quote by a specific user
        ctx.say("Not yet implemented").await?;
        return Ok(());
    } else {
        quote = get_random_quote().await;
    }

    if let Some(quote) = quote {
        ctx.send(|r| r.embed(|e| create_embed(e, quote)))
            .await
            .unwrap();
    }
    Ok(())
}

/// Get a list of locales.
#[poise::command(slash_command, prefix_command)]
pub async fn locales(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(LOCALES.join(", ")).await?;
    Ok(())
}
