use crate::db;
use crate::db::entity::quotes;
use sea_orm::{ActiveModelTrait, DbErr, Set};

pub async fn create_quote(
    guild_id: String,
    quoted_by: String,
    quote_string: String,
    author: String,
) -> Result<(), DbErr> {
    let conn = db::establish_connection().await?;

    let quote = quotes::ActiveModel {
        guild_id: Set(guild_id),
        quote_string: Set(quote_string),
        quoted_by: Set(quoted_by),
        author: Set(author),
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
