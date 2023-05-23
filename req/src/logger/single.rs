use super::complex::{Logger, LogType};

pub fn fatal(text: &str) -> ! {
    println!("FATAL: {text}");
    std::process::exit(0);
}

pub fn write(logger: &Option<Logger>, log_type: LogType, text: &str) {
    if logger.is_some() {
        logger.as_ref().cloned().unwrap().log(log_type, String::from(text));
    }
}
