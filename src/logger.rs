use std::io::{self, Write};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub(crate) enum LogLevel {
    Info,
    Warning,
    Error,
}

pub(crate) fn log(level: LogLevel, message: &str) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut color_spec = ColorSpec::new();
    match level {
        LogLevel::Info => color_spec.set_fg(Some(Color::Green)),
        LogLevel::Warning => color_spec.set_fg(Some(Color::Yellow)),
        LogLevel::Error => color_spec.set_fg(Some(Color::Red)),
    };

    stdout.set_color(&color_spec)?;
    write!(&mut stdout, "[{}] ", match level {
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR",
    })?;
    stdout.reset()?;
    writeln!(&mut stdout, "{}", message)?;
    Ok(())
}
