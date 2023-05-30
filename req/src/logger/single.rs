pub fn fatal(text: &str) -> ! {
    println!("FATAL: {text}");
    std::process::exit(0);
}
