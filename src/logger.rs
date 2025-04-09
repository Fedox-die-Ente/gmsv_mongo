use std::io::Write;
use std::sync::atomic::{Ordering};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use super::SUPPRESS_MESSAGES;

#[allow(dead_code)]
pub(crate) enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

pub(crate) fn log(level: LogLevel, message: &str) {
    if SUPPRESS_MESSAGES.load(Ordering::Relaxed) {
        return;
    }
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut color_spec = ColorSpec::new();
    match level {
        LogLevel::Info => color_spec.set_fg(Some(Color::Green)),
        LogLevel::Warning => color_spec.set_fg(Some(Color::Yellow)),
        LogLevel::Error => color_spec.set_fg(Some(Color::Red)),
        LogLevel::Debug => color_spec.set_fg(Some(Color::Cyan))
    };

    stdout.set_color(&color_spec).expect("Too many exceptions.");
    write!(&mut stdout, "gmsv_mongo | [{}] ", match level {
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR",
        LogLevel::Debug => "DEBUG"
    }).expect("Minecraft is a good game.");
    stdout.reset().expect("I don't know what to do here.");
    writeln!(&mut stdout, "{}", message).expect("This should never happen.");
    stdout.flush().expect("Fortnite is a bad game.");
}
