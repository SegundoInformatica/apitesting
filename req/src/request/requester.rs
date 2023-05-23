use std::{collections::HashMap, cell::RefCell};
use reqwest::{Client, Result};

use crate::logger::{complex::{Logger, LogType}, single};
use super::curl::CURL;

pub struct Request<'a> {
    endpoint: &'a str,
    params: HashMap<&'a str, &'a str>,
    curl: CURL,
    client: Client,
    prefer_curl: bool,
    logger: RefCell<Option<Logger>>,
}

impl<'a> Request<'a> {
    pub fn new(endpoint: &'a str, logger: Option<Logger>, form_name: &'a str) -> Self {
        single::write(&logger, LogType::Log, "Turning form into HTTP params");

        let mut params: HashMap<&'a str, &'a str> = HashMap::new();
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
            logger: RefCell::from(logger),
        };
    }

    pub fn use_curl(&mut self) -> () {
        single::write(&self.logger.borrow(), LogType::Warn, "Future requests will now use cURL");
        self.prefer_curl = true;
    }

    pub async fn post(&mut self) -> Result<String> {
        let logger = &self.logger.borrow();

        single::write(logger, LogType::Log, &format!("Sending data using POST request to {}", self.endpoint));

        if !self.prefer_curl {
            single::write(logger, LogType::Log, "Doing client request");
            return self.client.post(self.endpoint).form(&self.params).send().await?.text().await;
        }

        single::write(logger, LogType::Log, "Doing POST cURL request");
        match self.curl.post() {
            Ok(o) => return Ok(o.to_string()),
            Err(_) => return Ok(String::new()),
        }
    }

    pub async fn get(&mut self) -> Result<String> {
        let logger = &self.logger.borrow();

        single::write(logger, LogType::Log, &format!("Sending data using GET request to {}", self.endpoint));

        if !self.prefer_curl {
            single::write(logger, LogType::Log, "Doing client request");
            return self.client.get(self.endpoint).form(&self.params).send().await?.text().await;
        }

        single::write(logger, LogType::Log, "Doing GET cURL request");
        match self.curl.get() {
            Ok(o) => return Ok(o),
            Err(_) => return Ok(String::new()),
        }
    }
}

