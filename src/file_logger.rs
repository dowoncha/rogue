use std::sync::{Arc, Mutex};
use std::io::prelude::*;

use std::panic;
use backtrace::Backtrace;

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

    // Panic hook
    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            error!("panic occurred: {}", s);
        } else if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            error!("panic occurred: {}", s);
        }

        if let Some(location) = panic_info.location() {
            error!("panic occurred in file '{}' at line {}", location.file(),
                location.line());
        } else {
            error!("panic occurred but can't get location information...");
        }

        let bt = Backtrace::new();

        error!("{:?}", bt);

        // error!("Panic: {}", panic_info);
    }));

    Ok(())
}