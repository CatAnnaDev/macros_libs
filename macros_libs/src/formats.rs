
#[macro_export]
macro_rules! fmt_join {
    ($iter:expr, $sep:expr) => {{
        $iter.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join($sep)
    }};
}

#[macro_export]
macro_rules! fmt_pad {
    ($s:expr, $width:expr) => {{
        format!("{:width$}", $s, width = $width)
    }};
    ($s:expr, $width:expr, right) => {{
        format!("{:>width$}", $s, width = $width)
    }};
    ($s:expr, $width:expr, left) => {{
        format!("{:<width$}", $s, width = $width)
    }};
}

#[macro_export]
macro_rules! fmt_titlecase {
    ($s:expr) => {{
        let mut c = $s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }};
}

#[macro_export]
macro_rules! fmt_repeat {
    ($s:expr, $n:expr) => {{
        std::iter::repeat($s).take($n).collect::<String>()
    }};
}

/// Encadre un texte multi-lignes `$text` dans une boîte ASCII avec bordures.
///
/// # Exemple
/// ```
/// println!("{}", fmt_boxed!("Hello\nWorld"));
/// /* Affiche :
/// +-------+
/// | Hello |
/// | World |
/// +-------+
/// */
/// ```
#[macro_export]
macro_rules! fmt_boxed {
    ($text:expr) => {{
        let lines: Vec<_> = $text.lines().collect();
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        let mut out = String::new();
        out += &format!("+{}+\n", "-".repeat(width + 2));
        for line in lines {
            out += &format!("| {:width$} |\n", line, width = width);
        }
        out += &format!("+{}+", "-".repeat(width + 2));
        out
    }};
}

/// Applique un style ANSI `$style` (codes couleur/format terminal) sur une chaîne formatée.
///
/// # Exemple
/// ```
/// println!("{}", styled_format!("31", "Error: {}", "fail"));
/// // Affiche en rouge "Error: fail"
/// ```
#[macro_export]
macro_rules! styled_format {
    ($style:expr, $($arg:tt)*) => {{
        format!("\x1b[{}m{}\x1b[0m", $style, format!($($arg)*))
    }};
}

/// Simple moteur de template : remplace dans `$text` les occurrences de `{{key}}` par `val.to_string()`.
///
/// # Exemple
/// ```
/// let s = template!("Hello {{name}}!", { "name" => "Alice" });
/// assert_eq!(s, "Hello Alice!");
/// ```
#[macro_export]
macro_rules! template {
    ($text:expr, { $($key:expr => $val:expr),* $(,)? }) => {{
        let mut s = $text.to_string();
        $(
            s = s.replace(&format!("{{{{{}}}}}", $key), &$val.to_string());
        )*
        s
    }};
}

/// Formatte un tableau simple (slice de slice de &str) en colonnes alignées à gauche,
/// avec 2 espaces entre colonnes.
///
/// # Exemple
/// ```
/// let table = &[
///     &["Nom", "Age", "Ville"],
///     &["Alice", "30", "Paris"],
///     &["Bob", "25", "Lyon"],
/// ];
/// println!("{}", format_table!(table));
/// /*
/// Nom    Age  Ville  
/// Alice  30   Paris  
/// Bob    25   Lyon   
/// */
/// ```
#[macro_export]
macro_rules! format_table {
    ($rows:expr) => {{
        let mut col_widths = vec![];
        for row in &$rows {
            for (i, cell) in row.iter().enumerate() {
                let width = cell.len();
                if col_widths.len() <= i {
                    col_widths.push(width);
                } else if col_widths[i] < width {
                    col_widths[i] = width;
                }
            }
        }

        let mut output = String::new();
        for row in $rows {
            for (i, cell) in row.iter().enumerate() {
                let padding = col_widths[i] - cell.len();
                output.push_str(cell);
                output.push_str(&" ".repeat(padding + 2));
            }
            output.push('\n');
        }
        output
    }};
}

/// Formatte un tableau ASCII avec bordures (+---+ style),
/// adapté pour affichage en console.
///
/// # Exemple
/// ```
/// let table = &[
///     &["ID", "Nom", "Score"],
///     &["1", "Alice", "90"],
///     &["2", "Bob", "85"],
/// ];
/// println!("{}", format_table_ascii!(table));
/// /*
/// +----+-------+-------+
/// | 1  | Alice | 90    |
/// | 2  | Bob   | 85    |
/// +----+-------+-------+
/// */
/// ```
#[macro_export]
macro_rules! format_table_ascii {
    ($rows:expr) => {{
        let mut col_widths = vec![];
        for row in &$rows {
            for (i, cell) in row.iter().enumerate() {
                let width = cell.len();
                if col_widths.len() <= i {
                    col_widths.push(width);
                } else if col_widths[i] < width {
                    col_widths[i] = width;
                }
            }
        }

        let mut output = String::new();

        let sep: String = col_widths.iter().map(|w| format!("+{}",
            "-".repeat(*w + 2)
        )).collect::<Vec<_>>().join("") + "+\n";
        output.push_str(&sep);

        for row in $rows {
            output.push('|');
            for (i, cell) in row.iter().enumerate() {
                let padding = col_widths[i] - cell.len();
                output.push_str(&format!(" {}{} |", cell, " ".repeat(padding)));
            }
            output.push('\n');
            output.push_str(&sep);
        }

        output
    }};
}

/// Formatte un tableau au format Markdown.
///
/// # Exemple
/// ```
/// let table = &[
///     &["Nom", "Age"],
///     &["Alice", "30"],
///     &["Bob", "25"],
/// ];
/// println!("{}", format_table_md!(table));
/// /*
/// | Nom   | Age |
/// | ----- | --- |
/// | Alice | 30  |
/// | Bob   | 25  |
/// */
/// ```
#[macro_export]
macro_rules! format_table_md {
    ($rows:expr) => {{
        let mut col_widths = vec![];
        for row in &$rows {
            for (i, cell) in row.iter().enumerate() {
                let width = cell.len();
                if col_widths.len() <= i {
                    col_widths.push(width);
                } else if col_widths[i] < width {
                    col_widths[i] = width;
                }
            }
        }

        let mut output = String::new();
        for (i, row) in $rows.iter().enumerate() {
            output.push('|');
            for (j, cell) in row.iter().enumerate() {
                let padding = col_widths[j] - cell.len();
                output.push_str(&format!(" {}{} |", cell, " ".repeat(padding)));
            }
            output.push('\n');

            if i == 0 {
                output.push('|');
                for w in &col_widths {
                    output.push_str(&format!(" {} |", "-".repeat(*w)));
                }
                output.push('\n');
            }
        }

        output
    }};
}

/// Affiche une liste de chaînes en colonnes alignées, avec `$cols` colonnes.
///
/// # Exemple
/// ```
/// let items = vec!["apple", "banana", "pear", "grape"];
/// println!("{}", columnize!(items, 2));
/// /*
/// apple   banana  
/// pear    grape   
/// */
/// ```
#[macro_export]
macro_rules! columnize {
    ($items:expr, $cols:expr) => {{
        let mut out = String::new();
        let max_len = $items.iter().map(|s| s.len()).max().unwrap_or(0);
        let col_width = max_len + 2;

        for (i, item) in $items.iter().enumerate() {
            out.push_str(&format!("{:<width$}", item, width = col_width));
            if (i + 1) % $cols == 0 {
                out.push('\n');
            }
        }

        if !$items.is_empty() && $items.len() % $cols != 0 {
            out.push('\n');
        }

        out
    }};
}

/// Formatte un tableau avec alignement variable par colonne (droite ou gauche).
///
/// `$align` est un slice de `&str` pouvant contenir `"right"` ou autre (pour left).
///
/// # Exemple
/// ```
/// let table = &[
///     &["Nom", "Age", "Ville"],
///     &["Alice", "30", "Paris"],
///     &["Bob", "25", "Lyon"],
/// ];
/// let align = &["left", "right", "left"];
/// println!("{}", format_align_table!(table, align));
/// /*
/// Nom    30  Paris  
/// Alice  30  Paris  
/// Bob    25  Lyon   
/// */
/// ```
#[macro_export]
macro_rules! format_align_table {
    ($rows:expr, $align:expr) => {{
        let mut col_widths = vec![];
        for row in &$rows {
            for (i, cell) in row.iter().enumerate() {
                let width = cell.len();
                if col_widths.len() <= i {
                    col_widths.push(width);
                } else if col_widths[i] < width {
                    col_widths[i] = width;
                }
            }
        }

        let mut out = String::new();
        for row in $rows {
            for (i, cell) in row.iter().enumerate() {
                let width = col_widths[i];
                match $align.get(i).map(|s| *s) {
                    Some("right") => out.push_str(&format!("{:>width$}  ", cell, width = width)),
                    _ => out.push_str(&format!("{:<width$}  ", cell, width = width)),
                }
            }
            out.push('\n');
        }
        out
    }};
}

