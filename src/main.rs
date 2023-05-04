use colored::Colorize;
use fern::colors::{Color, ColoredLevelConfig};
use migration::{Migrator, MigratorTrait};
use modules::{quotes::queries::get_random_quote, *};
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
        .level_for("skipbot_rust", log::LevelFilter::Info)
        .apply()
}

async fn setup_bot_framework() -> FrameworkResult {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: get_enabled_commands(),
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

    // Database migration
    match db::establish_connection().await {
        Ok(conn) => {
            log::info!("Connection to database successful");

            if let Err(e) = Migrator::up(&conn, None).await {
                log::error!("Database migration failed");
                log::error!("{}", e);
            } else {
                log::info!("Database migration successful")
            }
        }
        Err(e) => {
            log::error!("Connection to database failed");
            log::error!("{}", e);
        }
    }

    let framework = setup_bot_framework().await;
    framework.run().await.unwrap();
}
