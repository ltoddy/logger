use std::io::Write;
use std::sync::Mutex;

use colored::Colorize;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

use crate::writer::Writer;

pub mod writer;

pub struct Logger {
    level: LevelFilter,
    writer: Writer,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            level: LevelFilter::Info,
            writer: Default::default(),
        }
    }

    pub fn init(self) -> Result<(), SetLoggerError> {
        let max_level = self.level;

        log::set_max_level(max_level);
        log::set_boxed_logger(Box::new(self))?;

        Ok(())
    }

    pub fn with_level(mut self, level: LevelFilter) -> Self {
        self.level = level;
        self
    }

    pub fn with_writer(mut self, writer: Writer) -> Self {
        self.writer = writer;
        self
    }

    fn write_to_stdout(&self, colored: bool, record: &Record) {
        let level = level(colored, record.level());

        let message = format!("{level}");
        println!("{message}");
    }

    fn write_to_stderr(&self, colored: bool, record: &Record) {
        let level = level(colored, record.level());

        let message = format!("{level}");
        eprintln!("{message}");
    }

    fn writer_to_other(&self, record: &Record, writer: &Box<Mutex<dyn Write + Sync + Send>>) {
        let level = level(false, record.level());

        let message = format!("{level}");

        if let Ok(mut writer) = writer.lock() {
            _ = writeln!(writer, "{message}");
        }
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        match self.writer {
            Writer::Stdout(colored) => self.write_to_stdout(colored, record),
            Writer::Stderr(colored) => self.write_to_stderr(colored, record),
            Writer::Other(ref writer) => self.writer_to_other(record, writer),
        }
    }

    fn flush(&self) {}
}

fn level(colored: bool, level: Level) -> String {
    #[cfg(feature = "colors")]
    {
        if colored {
            match level {
                Level::Error => format!("{:<5}", level.to_string().red()),
                Level::Warn => format!("{:<5}", level.to_string().yellow()),
                Level::Info => format!("{:<5}", level.to_string().cyan()),
                Level::Debug => format!("{:<5}", level.to_string().purple()),
                Level::Trace => format!("{:<5}", level.to_string().normal()),
            }
        } else {
            format!("{:<5}", level.to_string())
        }
    }
    #[cfg(not(feature = "colors"))]
    {
        format!("{:<5}", record.level().to_string())
    }
}
