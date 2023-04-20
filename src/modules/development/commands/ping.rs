use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tracing::info;

pub fn run(_options: &[CommandDataOption]) {
    info!("ping command ran")
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("Replies with 'pong'")
}
