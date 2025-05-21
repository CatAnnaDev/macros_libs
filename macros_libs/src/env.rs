#[macro_export]
macro_rules! env_or {
    ($key:expr, $default:expr) => {{ std::env::var($key).unwrap_or_else(|_| $default.to_string()) }};
}

#[macro_export]
macro_rules! get_env {
    ($key:expr) => {{
        std::env::var($key).ok()
    }};
}
