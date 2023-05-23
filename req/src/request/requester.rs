use std::collections::HashMap;
use reqwest::{Client, Result};

use crate::logger::{complex::{Logger, LogType}, single};
use super::curl::CURL;

pub struct Request {
    endpoint: &'static str,
    params: HashMap<&'static str, &'static str>,
    curl: CURL,
    client: Client,
    prefer_curl: bool,
    logger: Option<Logger>,
}

impl Request {
    pub fn new(endpoint: &'static str, logger: Option<Logger>, form_name: &'static str) -> Self {
        single::write(&logger, LogType::Log, "Turning form into HTTP params");

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("name", form_name);

        let data = params.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<String>>().join("&");

        single::write(&logger, LogType::Log, &format!("Params concluded: ?{}", data));
        single::write(&logger, LogType::Log, "Creating Request object");

        return Self {
            endpoint,
            params,
            curl: CURL::new(endpoint.to_string(), data),
            client: Client::new(),
            prefer_curl: false,
            logger,
        };
    }

    pub fn use_curl(&mut self) -> () {
        single::write(&self.logger, LogType::Warn, "Future requests will now use cURL");
        self.prefer_curl = true;
    }

    pub async fn post(&mut self) -> Result<String> {
        single::write(&self.logger, LogType::Log, &format!("Sending data using POST request to {}", self.endpoint));

        if !self.prefer_curl {
            single::write(&self.logger, LogType::Log, "Doing client request");
            return self.client.post(self.endpoint).form(&self.params).send().await?.text().await;
        }

        single::write(&self.logger, LogType::Log, "Doing POST cURL request");
        match self.curl.post() {
            Ok(o) => return Ok(o.to_string()),
            Err(_) => return Ok(String::new()),
        }
    }

    pub async fn get(&mut self) -> Result<String> {
        single::write(&self.logger, LogType::Log, &format!("Sending data using GET request to {}", self.endpoint));

        if !self.prefer_curl {
            single::write(&self.logger, LogType::Log, "Doing client request");
            return self.client.get(self.endpoint).form(&self.params).send().await?.text().await;
        }

        single::write(&self.logger, LogType::Log, "Doing GET cURL request");
        match self.curl.get() {
            Ok(o) => return Ok(o),
            Err(_) => return Ok(String::new()),
        }
    }
}

