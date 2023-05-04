use crate::db;
use crate::db::entity::quotes;
use sea_orm::{ActiveModelTrait, DbErr, Set};

pub async fn create_quote(
    guild_id: &str,
    quoted_by: &str,
    quote_string: &str,
    author: &str,
) -> Result<(), DbErr> {
    let conn = db::establish_connection().await?;

    let quote = quotes::ActiveModel {
        guild_id: Set(guild_id.to_owned()),
        quote_string: Set(quote_string.to_owned()),
        quoted_by: Set(quoted_by.to_owned()),
        author: Set(author.to_owned()),
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
