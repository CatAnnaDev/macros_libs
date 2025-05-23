#![crate_type = "proc-macro"]

#[macro_use]
#[cfg(feature = "loggings")]
pub mod logging;

#[macro_use]
#[cfg(feature = "control")]
pub mod control;

#[macro_use]
#[cfg(feature = "time")]
pub mod time;

#[macro_use]
#[cfg(feature = "env")]
pub mod env;

#[macro_use]
#[cfg(feature = "debug")]
pub mod debug;

#[macro_use]
#[cfg(feature = "util")]
pub mod util;

#[macro_use]
#[cfg(feature = "collection")]
pub mod collection;

#[macro_use]
#[cfg(feature = "assert")]
pub mod assert;

#[macro_use]
#[cfg(feature = "formats")]
pub mod formats;
