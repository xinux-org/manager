use log::{Level, Log, Metadata, Record};

pub struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{} -{} {}",
                record.level(),
                record
                    .module_path()
                    .map_or_else(|| "".to_string(), |v| format!(" {}:", v)),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
