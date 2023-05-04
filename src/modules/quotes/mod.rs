pub mod commands;
pub mod queries;

pub use commands::*;

use super::types::SkipbotCommandGroup;

pub fn get_module_commands() -> SkipbotCommandGroup {
    vec![quote(), inspireme(), locales()]
}
