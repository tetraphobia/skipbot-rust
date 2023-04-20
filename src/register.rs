#[macro_export]
macro_rules! register_command {
    ($http:expr, $register_func:expr) => {
        Command::create_global_application_command($http, |c| $register_func(c))
    };
}
