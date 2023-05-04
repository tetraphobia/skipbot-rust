use crate::db;
use crate::db::entity::{quotes, quotes::Model};
use migration::Expr;
use poise::serenity_prelude::User;
use sea_orm::{ActiveModelTrait, DbBackend, DbErr, EntityTrait, QueryOrder, Set, Statement};

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

pub async fn get_random_quote() -> Option<Model> {
    let conn = db::establish_connection().await.unwrap();
    log::info!("Picking random quote");

    let quote = quotes::Entity::find()
        .from_raw_sql(Statement::from_string(
            DbBackend::Sqlite,
            r#"SELECT * FROM quotes ORDER BY RANDOM() LIMIT 1"#.to_owned(),
        ))
        .one(&conn)
        .await;

    if let Err(e) = quote {
        log::error!("Error while fetching random quote");
        log::error!("{:#?}", e);
        return None;
    }

    quote.unwrap()
}
