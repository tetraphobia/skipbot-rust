use fern::colors::{Color, ColoredLevelConfig};
use modules::*;
use poise::serenity_prelude as serenity;
use std::time::SystemTime;

mod modules;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

fn setup_logger() {
    let colors = ColoredLevelConfig::new().debug(Color::Magenta);

    fern::Dispatch::new()
        .chain(std::io::stdout())
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                // This will color the log level only, not the whole line. Just a touch.
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("tracing::span", log::LevelFilter::Warn)
        .level_for("serenity", log::LevelFilter::Warn)
        .apply()
        .unwrap();
}

#[tokio::main]
async fn main() {
    setup_logger();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: development::get_module_commands(),
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    log::info!("Skipbot framework initialized");
    framework.run().await.unwrap();
}
