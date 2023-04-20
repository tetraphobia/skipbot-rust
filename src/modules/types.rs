pub type SkipbotCommandGroup = Vec<
    poise::Command<crate::Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>,
>;
