extern crate log;

use log::{
    Log,
    Level,
    Metadata,
    Record,
    SetLoggerError
};

struct Logger {
    level: Level,
}

#[allow(dead_code)]
static LOGGER: Logger = Logger { level: Level::Trace };

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
            // ensure clean output in raw console mode
            print!("\r");
        }
    }

    fn flush(&self) {
        // nothing to do here
    }
}

/// Initializes the global logger with a Logger instance with
/// `max_log_level` set to a specific log level.
///
/// ```
/// # #[macro_use] extern crate log;
/// # extern crate ophelia_log;
/// #
/// # fn main() {
/// ophelia_log::init_with_level(log::Level::Warn).unwrap();
///
/// warn!("This is an example message.");
/// info!("This message will not be logged.");
/// # }
/// ```
#[allow(dead_code)]
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(level.to_level_filter()))
}

/// Initializes the global logger with a MicoLog instance with
/// `max_log_level` set to `LogLevel::Trace`.
///
/// ```
/// # #[macro_use] extern crate log;
/// # extern crate ophelia_log;
/// #
/// # fn main() {
/// ophelia_log::init().unwrap();
/// warn!("This is an example message.");
/// # }
/// ```
#[allow(dead_code)]
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Info)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_log() {
        init();
        log::info!("hello, world!");
        assert!(true);
    }
}
