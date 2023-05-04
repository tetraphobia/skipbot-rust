use self::types::SkipbotCommandGroup;

pub mod development;
pub mod quotes;

pub mod types;

pub fn get_enabled_commands() -> SkipbotCommandGroup {
    let mut enabled = vec![];

    enabled.append(&mut development::get_module_commands());
    enabled.append(&mut quotes::get_module_commands());

    log::debug!("{:#?}", &enabled);

    enabled
}
