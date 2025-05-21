#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),*) => {{
        vec![$($x.to_string()),*]
    }};
}

#[macro_export]
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}
