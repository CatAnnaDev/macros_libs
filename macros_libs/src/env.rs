#[macro_export]
macro_rules! env_or {
    ($key:expr, $default:expr) => {{ std::env::var($key).unwrap_or_else(|_| $default.to_string()) }};
}
