#[macro_export]
macro_rules! time {
    ($label:expr, $block:block) => {{
        let now = Instant::now();
        let result = $block;
        let elapsed = now.elapsed();
        $crate::log_info!("{} took {:?}", $label, elapsed);
        result
    }};
}
