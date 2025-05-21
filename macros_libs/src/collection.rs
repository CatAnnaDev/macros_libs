/// Crée un `Vec<String>` à partir de littéraux.
#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),*) => {{
        vec![$($x.to_string()),*]
    }};
}

/// Crée une `HashMap` à partir de paires clé => valeur.
#[macro_export]
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}