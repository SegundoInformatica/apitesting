use std::{collections::HashMap, process::exit};
use reqwest::{Client, Result};

struct NewRequest {
    endpoint: &'static str,
    params: HashMap<&'static str, &'static str>,
}

impl NewRequest {
    pub fn new(endpoint: &'static str, form_name: &'static str) -> Self {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("name", form_name);

        return Self {
            endpoint,
            params,
        };
    }

    pub async fn post(&self) -> Result<String> {
        return Client::new().post(self.endpoint).form(&self.params).send().await?.text().await;
    }

    pub async fn get(&self) -> Result<String> {
        return Client::new().get(self.endpoint).form(&self.params).send().await?.text().await;
    }
}

#[tokio::main]
async fn main() {
    let tobias = NewRequest::new("http://127.0.0.1:5000", "Tobias");
    let null = NewRequest::new("http://127.0.0.1:5000", "");

    println!("POST TOBIAS: {}", tobias.post().await.expect("Could not fetch text from request").trim());
    println!("POST NULL:   {}", null.post().await.expect("Could not fetch text from request").trim());
    println!("GET TOBIAS: {}", tobias.get().await.expect("Could not fetch text from request").trim());
    println!("GET NULL:   {}", null.get().await.expect("Could not fetch text from request").trim());

    exit(0);
}
