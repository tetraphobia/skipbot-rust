use anyhow::anyhow;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;

mod handler;
mod modules;
mod register;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let client = Client::builder(&token, GatewayIntents::non_privileged())
        .event_handler(handler::Handler)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
