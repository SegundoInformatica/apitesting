use std::{io, process::Command};

pub struct CURL {
    data: String,
    endpoint: String,
}

impl CURL {
    pub fn new(endpoint: String, data: String) -> Self {
        return Self {
            data,
            endpoint,
        };
    }

    pub fn get(&mut self) -> Result<String, io::Error> {
        let mut command = Command::new("curl");
        command.args(["-X", "GET", &self.endpoint, "-d", &self.data]);

        return match command.output() {
            Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).to_string()),
            Err(e) => Err(e),
        };
    }

    pub fn post(&mut self) -> Result<String, io::Error> {
        let mut command = Command::new("curl");
        command.args(["-X", "POST", &self.endpoint, "-d", &self.data]);

        return match command.output() {
            Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).to_string()),
            Err(e) => Err(e),
        };
    }

}
