use std::collections::HashMap;
use reqwest::{Client, Result};

pub struct Request {
    endpoint: &'static str,
    params: HashMap<&'static str, &'static str>,
    client: Client,
    prefer_curl: bool,
}

impl Request {
    pub fn new(endpoint: &'static str, form_name: &'static str) -> Self {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("name", form_name);

        return Self {
            endpoint,
            params,
            client: Client::new(),
            prefer_curl: false,
        };
    }

    pub fn use_curl(&mut self) -> () {
        self.prefer_curl = true;
    }

    pub async fn post(&self) -> Result<String> {
        if !self.prefer_curl {
            return self.client.post(self.endpoint).form(&self.params).send().await?.text().await;
        }

        // TODO: Make cURL act
        return Ok(String::new());
    }

    pub async fn get(&self) -> Result<String> {
        if !self.prefer_curl {
            return self.client.get(self.endpoint).form(&self.params).send().await?.text().await;
        }

        // TODO: Make cURL act
        return Ok(String::new());
    }
}

