use crate::db::entity::quotes::Model;
use crate::modules::quotes::queries::*;
use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, CreateEmbed, User};
use rand::seq::SliceRandom;

const LOCALES: [&str; 5] = ["en", "fr", "ja", "de", "zh-CN"];

fn create_embed<'a>(embed: &'a mut CreateEmbed, quote: Model) -> &'a mut CreateEmbed {
    embed
        .color(0xe43a25)
        .title(format!("Quoth the {}...", quote.author))
        .thumbnail(quote.author_avatar_url)
        .description(quote.quote_string)
        .field("Date Added", quote.created_at.unwrap(), true)
        .field("Added By", quote.quoted_by, true)
        .field("ID", quote.id, true)
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

    let (guild_id, quoted_by, quote_string, author): (String, String, String, User) = (
        ctx.guild_id().unwrap().to_string(),
        ctx.author().name.clone(),
        quote,
        user,
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

    // Will be used for reading the quotes.
    let locale = locale.unwrap_or_else(|| {
        LOCALES
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
            .to_string()
    });

    let guild_id: String = match global {
        Some(global_search) => {
            if !global_search {
                ctx.guild_id().unwrap().to_string()
            } else {
                "".to_string()
            }
        }
        _ => ctx.guild_id().unwrap().to_string(),
    };

    let quote = get_random_quote(guild_id, author).await;

    match quote {
        Some(quote) => {
            ctx.send(|r| r.embed(|e| create_embed(e, quote))).await?;
        }
        _ => {
            ctx.say("Something either went horribly wrong, or you have no quotes saved. Give the `/quote` command a try.").await?;
        }
    }
    Ok(())
}

/// Get a list of locales.
#[poise::command(slash_command, prefix_command)]
pub async fn locales(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(LOCALES.join(", ")).await?;
    Ok(())
}
