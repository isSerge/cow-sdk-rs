use eyre::Error;
use reqwest::Client as HttpClient;
use reqwest::Url;
use std::sync::Arc;

pub struct Client {
    client: Arc<HttpClient>,
}

impl Client {
    pub fn new(api_url: &str) -> Result<Self, Error> {
        let api_url = Url::parse(api_url).expect("Invalid RPC URL");
        Ok(Self {
            client: Arc::new(HttpClient::new()),
        })
    }
}
