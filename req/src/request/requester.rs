use std::collections::HashMap;
use reqwest::{Client, Result};

use crate::logger::complex::{Logger, LogType};
use super::curl::CURL;

pub struct Request<'a> {
    endpoint: &'a str,
    params: HashMap<&'a str, &'a str>,
    curl: CURL,
    client: Client,
    prefer_curl: bool,
    logger: Logger,
}

impl<'a> Request<'a> {
    pub fn new(endpoint: &'a str, logger: Logger, form_name: &'a str) -> Self {
        logger.log(LogType::Log, format!("Creating request at {endpoint}"));

        logger.log(LogType::Log, format!("Adding params to the request"));
        let mut params: HashMap<&'a str, &'a str> = HashMap::new();
        params.insert("name", form_name);

        let data = params.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<String>>().join("&");

        logger.log(LogType::Log, format!("Done creating request"));
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
        self.logger.log(LogType::Warn, String::from("Future requests will use cURL"));
        self.prefer_curl = true;
    }

    pub async fn post(&mut self) -> Result<String> {
        self.logger.log(LogType::Warn, format!("Throwing POST request at {}", self.endpoint));

        if !self.prefer_curl {
            self.logger.log(LogType::Log, String::from("Using native request"));
            return self.client.post(self.endpoint).form(&self.params).send().await?.text().await;
        }

        self.logger.log(LogType::Log, String::from("Using cURL request"));
        match self.curl.post() {
            Ok(o) => return Ok(o.to_string()),
            Err(_) => return Ok(String::new()),
        }
    }

    pub async fn get(&mut self) -> Result<String> {
        self.logger.log(LogType::Warn, format!("Throwing GET request at {}", self.endpoint));

        if !self.prefer_curl {
            self.logger.log(LogType::Log, String::from("Using native request"));
            return self.client.get(self.endpoint).form(&self.params).send().await?.text().await;
        }

        self.logger.log(LogType::Log, String::from("Using cURL request"));
        match self.curl.get() {
            Ok(o) => return Ok(o),
            Err(_) => return Ok(String::new()),
        }
    }
}

