use std::fs::OpenOptions;
use std::fmt::Display;
use std::io::{Seek, SeekFrom, Write};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};

static SAVE_LOG_TO_FILE: AtomicBool = AtomicBool::new(false);
static LOG_FILE_PATH: OnceLock<String> = OnceLock::new();
static MAX_LOG_FILE_SIZE: OnceLock<u64> = OnceLock::new();
static LOG_LEVEL_FILTER: OnceLock<Level> = OnceLock::new();

pub enum Level{
    Debug,
    Info,
    Warn,
    Error,
    Success,
}

impl Display for Level{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Debug => write!(f, "Debug"),
            Level::Info => write!(f, "Info"),
            Level::Warn => write!(f, "Warn"),
            Level::Error => write!(f, "Error"),
            Level::Success => write!(f, "Success"),
        }
    }
}

#[allow(dead_code)]
pub fn enable_log_saving(path: &str) {
    LOG_FILE_PATH.set(path.to_string()).ok();
    SAVE_LOG_TO_FILE.store(true, Ordering::SeqCst);
}

pub fn set_max_log_file_size(bytes: u64) {
    MAX_LOG_FILE_SIZE.set(bytes).ok();
}

pub fn set_log_level_filter(level: Level) {
    LOG_LEVEL_FILTER.set(level).ok();
}

#[doc(hidden)]
#[allow(dead_code)]
fn level_priority(level: &Level) -> usize {
    match level {
        Level::Error => 4,
        Level::Warn => 3,
        Level::Info => 2,
        Level::Debug => 1,
        Level::Success => 2,
    }
}

#[doc(hidden)]
#[allow(dead_code)]
pub fn should_log(level: &Level) -> bool {
    if let Some(filter) = LOG_LEVEL_FILTER.get() {
        level_priority(level) >= level_priority(filter)
    } else {
        true
    }
}

#[doc(hidden)]
pub fn write_log_to_file(line: &str) {
    if SAVE_LOG_TO_FILE.load(Ordering::SeqCst) {
        if let Some(path) = LOG_FILE_PATH.get() {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .read(true)
                .open(path)
            {
                if let Some(max) = MAX_LOG_FILE_SIZE.get() {
                    if let Ok(metadata) = file.metadata() {
                        if metadata.len() > *max {
                            let _ = file.set_len(0);
                            let _ = file.seek(SeekFrom::Start(0));
                        }
                    }
                }
                let _ = writeln!(file, "{}", line);
            }
        }
    }
}

/// Affiche un message de log avec niveau et couleur, puis le sauvegarde si activé.
#[macro_export]
macro_rules! log_with_level {
    ($level:expr, $color:expr, $($arg:tt)*) => {{
        if should_log($level) {
            let now = chrono::Local::now();
            let msg = format!("{} [{}] {}", now.format("%Y-%m-%d %H:%M:%S"), $level, format!($($arg)*));
            println!("\x1b[{}m{}\x1b[0m", $color, msg);
            write_log_to_file(&msg);
        }
    }};
}

/// Log INFO (bleu).
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        log_with_level!(&Level::Info, "34", $($arg)*);
    };
}

/// Log WARN (jaune).
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        log_with_level!(&Level::Warn, "33", $($arg)*);
    };
}

/// Log ERROR (rouge).
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log_with_level!(&Level::Error, "31", $($arg)*);
    };
}

/// Log DEBUG (gris), actif uniquement en debug.
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        log_with_level!(&Level::Debug, "90", $($arg)*);
    };
}

/// Log SUCCESS (vert).
#[macro_export]
macro_rules! log_success {
    ($($arg:tt)*) => {
        log_with_level!(&Level::Success, "32", $($arg)*);
    };
}
