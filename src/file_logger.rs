use std::sync::{Arc, Mutex};
use std::io::prelude::*;

lazy_static! {
    pub static ref LOGGER: FileLogger = {
        let file = std::fs::File::create("logs.txt")
            .expect("Failed to create log file");

        FileLogger {
            file: Arc::new(Mutex::new(file))
        }
    };
}

pub struct FileLogger {
    file: Arc<Mutex<std::fs::File>>
}

impl log::Log for FileLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool { 
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let file_m = Arc::clone(&self.file);

            let mut file = file_m.lock().expect("Poisoned file mutex");

            writeln!(
                file,
                "{} - {}",
                record.level(), 
                record.args()
            ).expect("Failed to write to file");
        }
    }

    fn flush(&self) {
        let file_m = Arc::clone(&self.file);

        let mut file = file_m.lock().unwrap();

        file.flush().expect("Failed to flush file");
    }
}

pub fn init() -> Result<(), Box<std::error::Error>> {
    log::set_logger(&*LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::max()))?;

    Ok(())
}