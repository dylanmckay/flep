//! Logging-related stuff.

use log::{self, LogRecord, LogLevelFilter, LogMetadata};

/// A trivial logger which simply prints to standard output.
struct SimpleLogger;

#[cfg(debug_assertions)]
const LOG_LEVEL_FILTER: LogLevelFilter = LogLevelFilter::Debug;
#[cfg(not(debug_assertions))]
const LOG_LEVEL_FILTER: LogLevelFilter = LogLevelFilter::Info;

/// Initializes the default logger.
pub fn initialize_default() -> Result<(), log::SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LOG_LEVEL_FILTER);
        Box::new(SimpleLogger)
    })
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LOG_LEVEL_FILTER.to_log_level().unwrap()
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}

