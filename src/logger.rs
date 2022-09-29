use lazy_static::lazy_static;
use slog::Logger;
use sloggers::terminal::{Destination, TerminalLoggerBuilder};
use sloggers::types::Severity;
use sloggers::Build;

#[cfg(not(debug_assertions))]
use sloggers::types::SourceLocation;

#[cfg(not(debug_assertions))]
fn get_logger() -> Logger {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Info);
    builder.destination(Destination::Stdout);
    builder.source_location(SourceLocation::None);

    let logger = builder.build().unwrap();
    logger
}

#[cfg(debug_assertions)]
fn get_logger() -> Logger {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    builder.destination(Destination::Stdout);

    builder.build().unwrap()
}

lazy_static! {
    pub static ref LOGGER: Logger = get_logger();
}
