#[macro_export]
macro_rules! cfg_print {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

/// Debug express d’une expression avec position.
#[macro_export]
macro_rules! dbg_expr {
    ($val:expr) => {{
        let val = $val;
        println!("[{}:{}] {} = {:?}", file!(), line!(), stringify!($val), &val);
        val
    }};
}