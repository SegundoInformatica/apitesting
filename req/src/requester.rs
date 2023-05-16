use std::collections::HashMap;
use reqwest::{Client, Result};

pub struct Request {
    endpoint: &'static str,
    params: HashMap<&'static str, &'static str>,
    client: Client,
}

impl Request {
    pub fn new(endpoint: &'static str, form_name: &'static str) -> Self {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("name", form_name);

        return Self {
            endpoint,
            params,
            client: Client::new(),
        };
    }

    pub async fn post(&self) -> Result<String> {
        return self.client.post(self.endpoint).form(&self.params).send().await?.text().await;
    }

    pub async fn get(&self) -> Result<String> {
        return self.client.get(self.endpoint).form(&self.params).send().await?.text().await;
    }
}

