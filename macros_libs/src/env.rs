/// Récupère une variable d’environnement, ou fait autre chose
#[macro_export]
macro_rules! env_or {
    ($key:expr, $default:expr) => {{ std::env::var($key).unwrap_or_else(|_| $default.to_string()) }};
}

/// Récupère une variable d’environnement, retourne `Option<String>`.
#[macro_export]
macro_rules! get_env {
    ($key:expr) => {{
        std::env::var($key).ok()
    }};
}