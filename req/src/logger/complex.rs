const LOG_TYPES: [&str; 3] = [
    "LOG",
    "WARN",
    "ERROR",
];

#[derive(num_enum::IntoPrimitive)]
#[repr(usize)]
pub enum LogType {
    Log,
    Warn,
    Error,
}

#[derive(Copy, Clone)]
pub struct Logger {
    log_dir: &'static str,
    debug: bool,
}

impl Logger {
    pub fn new() -> Self {
        return Self {
            log_dir: "",
            debug: false,
        };
    }

    pub fn set_debug(&mut self, to: Option<bool>) {
        if to.is_none() {
            self.debug = false;
            return;
        }

        self.debug = to.unwrap();
    }

    pub fn set_log_dir(&mut self, path: &'static str) {
        // TODO: Check for path
        self.log_dir = path;
    }

    pub fn log(self, log_type: LogType, text: String) {
        if self.debug {
            println!("{} » {}", LOG_TYPES.get(log_type.try_into().unwrap_or(0)).unwrap(), text);
        }
    }
}
