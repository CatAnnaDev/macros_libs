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

#[macro_export]
macro_rules! measure {
    ($block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        (result, start.elapsed())
    }};
}