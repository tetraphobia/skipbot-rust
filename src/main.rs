use colored::Colorize;
use fern::colors::{Color, ColoredLevelConfig};
use modules::*;
use poise::{serenity_prelude as serenity, FrameworkBuilder};
use std::time::SystemTime;

mod db;
mod modules;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

type FrameworkResult =
    FrameworkBuilder<Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>;

async fn setup_logger() -> Result<(), log::SetLoggerError> {
    let colors = ColoredLevelConfig::new()
        .debug(Color::Magenta)
        .info(Color::Blue);

    fern::Dispatch::new()
        .chain(std::io::stdout())
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now())
                    .to_string()
                    .white(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("tracing::span", log::LevelFilter::Warn)
        .level_for("serenity", log::LevelFilter::Warn)
        .level_for("skipbot_rust", log::LevelFilter::Debug)
        .apply()
}

async fn setup_bot_framework() -> FrameworkResult {
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

    log::info!("Framework successfully initialized");
    framework
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    match setup_logger().await {
        Ok(_) => log::info!("Logger configured successfully"),
        Err(error) => {
            println!("Error while setting up logging");
            println!("{}", error)
        }
    }

    match db::establish_connection().await {
        Ok(_) => log::info!("Database connection established"),
        Err(error) => {
            log::error!("Database connection failed");
            log::error!("{}", error)
        }
    }

    let framework = setup_bot_framework().await;
    framework.run().await.unwrap();
}
