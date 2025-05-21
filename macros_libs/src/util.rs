/// Répète une expression `n` fois.
///
/// # Exemple
/// ```
/// repeat_n!(3, println!("Hello"));
/// ```
#[macro_export]
macro_rules! repeat_n {
    ($n:expr, $body:expr) => {{
        for _ in 0..$n {
            $body;
        }
    }};
}

/// Applique un bloc à chaque élément d’un tableau inline.
///
/// # Exemple
/// ```
/// for_each!([1, 2, 3], x => {
///     println!("{}", x);
/// });
/// ```
#[macro_export]
macro_rules! for_each {
    ([$($item:expr),*], $var:ident => $body:block) => {{
        $(let $var = $item; $body)*
    }};
}

/// Sérialise une valeur en JSON (joli format).
///
/// # Exemple
/// ```
/// let s = to_json!(my_struct);
/// ```
#[macro_export]
macro_rules! to_json {
    ($val:expr) => {{
        serde_json::to_string_pretty(&$val).unwrap()
    }};
}

/// Désérialise un JSON en objet Rust.
///
/// # Exemple
/// ```
/// let obj: MyStruct = from_json!(json_str).unwrap();
/// ```
#[macro_export]
macro_rules! from_json {
    ($s:expr) => {{
        serde_json::from_str($s)
    }};
}

/// Enchaîne une série d'appels de méthodes sur une valeur.
///
/// # Exemple
/// ```
/// let s = chain!("hello" => trim::to_string);
/// ```
#[macro_export]
macro_rules! chain {
    ($val:expr => $($func:ident)::*) => {{
        $val$(.$func())*
    }};
}