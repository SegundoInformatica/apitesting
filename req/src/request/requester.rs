use std::collections::HashMap;
use reqwest::{Client, Result};

use super::curl::CURL;

pub struct Request {
    endpoint: &'static str,
    params: HashMap<&'static str, &'static str>,
    curl: CURL,
    client: Client,
    prefer_curl: bool,
}

impl Request {
    pub fn new(endpoint: &'static str, form_name: &'static str) -> Self {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("name", form_name);

        let data = params.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<String>>().join("&");

        return Self {
            endpoint,
            params,
            curl: CURL::new(endpoint.to_string(), data),
            client: Client::new(),
            prefer_curl: false,
        };
    }

    pub fn use_curl(&mut self) -> () {
        self.prefer_curl = true;
    }

    pub async fn post(&mut self) -> Result<String> {
        if !self.prefer_curl {
            return self.client.post(self.endpoint).form(&self.params).send().await?.text().await;
        }

        match self.curl.post() {
            Ok(o) => return Ok(o.to_string()),
            Err(_) => return Ok(String::new()),
        }
    }

    pub async fn get(&mut self) -> Result<String> {
        if !self.prefer_curl {
            return self.client.get(self.endpoint).form(&self.params).send().await?.text().await;
        }

        match self.curl.get() {
            Ok(o) => return Ok(o),
            Err(_) => return Ok(String::new()),
        }
    }
}

