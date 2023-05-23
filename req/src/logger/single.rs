use super::complex::{Logger, LogType};

pub fn fatal(text: &str) -> ! {
    println!("FATAL: {text}");
    std::process::exit(0);
}

pub fn write(logger: &Option<Logger>, log_type: LogType, text: &str) {
    if logger.is_some() {
        // NOTE: same thing i said in logger/complex.rs:15 "cloning in the long run is bad imo"
        logger.clone().unwrap().log(log_type, String::from(text));
    }
}
