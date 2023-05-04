use sea_orm::prelude::*;
use sea_orm::*;
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let url = env::var("DATABASE_URL").unwrap();

    let opt = ConnectOptions::new(url);
    Database::connect(opt).await
}
