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

#[macro_export]
macro_rules! zip_vecs {
    ($a:expr, $b:expr) => {{
        $a.into_iter().zip($b).collect::<Vec<_>>()
    }};
}

#[macro_export]
macro_rules! chunk_vec {
    ($vec:expr, $size:expr) => {{
        $vec.chunks($size).map(|c| c.to_vec()).collect::<Vec<_>>()
    }};
}

#[macro_export]
macro_rules! dedup_vec {
    ($vec:expr) => {{
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        $vec.into_iter().filter(|x| seen.insert(x.clone())).collect::<Vec<_>>()
    }};
}
