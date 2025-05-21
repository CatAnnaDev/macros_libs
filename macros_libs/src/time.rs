/// Mesure le temps d’un bloc et affiche la durée avec description.
#[macro_export]
macro_rules! time_it {
    ($desc:expr, $block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();
        println!("{} took: {:?}", $desc, duration);
        result
    }};
}

/// Mesure le temps d’un bloc et retourne `(résultat, durée)`.
#[macro_export]
macro_rules! measure {
    ($block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        (result, start.elapsed())
    }};
}