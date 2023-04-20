use serenity::model::application::command::Command;
use serenity::prelude::*;

use crate::modules::development::commands::*;
use crate::register_command;

pub async fn register_commands(ctx: Context) {
    register_command!(&ctx.http, ping::register).await.unwrap();
    register_command!(&ctx.http, github::register)
        .await
        .unwrap();
}
