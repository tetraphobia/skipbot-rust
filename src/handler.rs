use crate::commands;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{error, info};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!(
                "Received command {:#?} from user {:#?}",
                command.data.name, command.user.name
            );

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |res| {
                    res.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| msg.content(content))
                })
                .await
            {
                error!("Cannot respond to slash command: {}", why)
            }
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|c| commands::ping::register(c))
        })
        .await;

        let commands_list: Vec<String> = commands.unwrap().iter().map(|x| x.name.clone()).collect();

        info!("Initialized commands: {:#?}", commands_list)
    }
}
