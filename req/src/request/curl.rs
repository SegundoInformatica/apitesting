use std::{io, process::Command};

pub struct CURL {
    data: String,
    endpoint: String,
    curl_command: Command,
}

impl CURL{
    pub fn new(endpoint: String, data: String) -> Self {
        let command = Command::new("curl");

        return Self {
            data,
            endpoint,
            curl_command: command,
        };
    }

    pub fn get(&mut self) -> Result<String, io::Error> {
        self.curl_command.args(["-X", "GET", &self.endpoint, "-d", &self.data]);

        return match self.curl_command.output() {
            Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).to_string()),
            Err(e) => Err(e),
        };
    }

    pub fn post(&mut self) -> Result<String, io::Error> {
        self.curl_command.args(["-X", "POST", &self.endpoint, "-d", &self.data]);

        return match self.curl_command.output() {
            Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).to_string()),
            Err(e) => Err(e),
        };
    }

}
