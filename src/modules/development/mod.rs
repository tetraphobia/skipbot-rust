pub mod commands;

pub use commands::*;

use super::types::SkipbotCommandGroup;

pub fn get_module_commands() -> SkipbotCommandGroup {
    vec![ping()]
}
