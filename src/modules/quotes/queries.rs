use crate::db;
use crate::db::entity::{quotes, quotes::Model};
use poise::serenity_prelude::User;
use rand::seq::SliceRandom;
use sea_orm::*;

const FALLBACK_AVATAR_URL: &str =
    "https://www.gravatar.com/avatar/00000000000000000000000000000000";

pub async fn create_quote(
    guild_id: String,
    quoted_by: String,
    quote_string: String,
    user: User,
) -> Result<(), DbErr> {
    let conn = db::establish_connection().await?;

    let quote = quotes::ActiveModel {
        guild_id: Set(guild_id),
        quote_string: Set(quote_string),
        quoted_by: Set(quoted_by),
        author: Set(user.name.to_owned()),
        author_avatar_url: Set(user
            .avatar_url()
            .unwrap_or_else(|| FALLBACK_AVATAR_URL.to_string())),
        ..Default::default()
    };

    match quote.insert(&conn).await {
        Ok(quote) => {
            log::info!("Created new Quote: {:#?}", quote);
            Ok(())
        }
        Err(e) => {
            log::error!("Failed to create new Quote: {:#?}", e);
            Err(e)
        }
    }
}

pub async fn get_random_quote(guild_id: String, author: Option<User>) -> Option<Model> {
    let conn = db::establish_connection().await.unwrap();
    log::info!("Picking random quote");

    let author_name: String;

    if let Some(author) = author {
        author_name = author.name;
    } else {
        author_name = "".to_owned();
    }

    let quotes = quotes::Entity::find()
        .filter(quotes::Column::GuildId.contains(&guild_id))
        .filter(quotes::Column::Author.contains(&author_name))
        .all(&conn)
        .await
        .unwrap();

    if quotes.len() == 0 {
        return None;
    };

    let picked = quotes.choose(&mut rand::thread_rng()).unwrap();

    println!("{:?}", &picked);
    Some(picked.to_owned())
}
