#[macro_export]
macro_rules! repeat_n {
    ($n:expr, $body:expr) => {{
        for _ in 0..$n {
            $body;
        }
    }};
}

#[macro_export]
macro_rules! for_each {
    ([$($item:expr),*], $var:ident => $body:block) => {{
        $(let $var = $item; $body)*
    }};
}

#[macro_export]
macro_rules! to_json {
    ($val:expr) => {{ serde_json::to_string_pretty(&$val).unwrap() }};
}

#[macro_export]
macro_rules! from_json {
    ($s:expr) => {{ serde_json::from_str($s) }};
}

#[macro_export]
macro_rules! chain {
    ($val:expr => $($func:ident)::*) => {{
        $val$(.$func())*
    }};
}
