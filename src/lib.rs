use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

pub struct Logger {
    level: LevelFilter,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            level: LevelFilter::Info,
        }
    }

    pub fn init(self) -> Result<(), SetLoggerError> {
        let max_level = self.level;

        log::set_max_level(max_level);
        log::set_boxed_logger(Box::new(self))?;

        Ok(())
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

        let message = format!("{}", record.args());
        println!("{message}");
    }

    fn flush(&self) {}
}
