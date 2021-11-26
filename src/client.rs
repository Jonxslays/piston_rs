use std::error::Error;

use reqwest::header::{HeaderMap, HeaderValue};

use super::Executor;
use super::ExecutorResponse;
use super::Runtime;

#[derive(Debug)]
pub struct Client {
    url: String,
    client: reqwest::Client,
    headers: HeaderMap,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            url: "https://emkc.org/api/v2/piston".to_string(),
            client: reqwest::Client::new(),
            headers: Self::generate_headers(None),
        }
    }

    pub fn new_with_key(key: &str) -> Self {
        Self {
            url: "https://emkc.org/api/v2/piston".to_string(),
            client: reqwest::Client::new(),
            headers: Self::generate_headers(Some(key)),
        }
    }

    fn generate_headers(key: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::with_capacity(3);
        headers.insert("Accept", HeaderValue::from_str("application/json").unwrap());
        headers.insert("User-Agent", HeaderValue::from_str("piston-rs").unwrap());

        if let Some(k) = key {
            headers.insert("Authorization", HeaderValue::from_str(k).unwrap());
        };

        headers
    }

    pub async fn fetch_runtimes(&self) -> Result<Vec<Runtime>, Box<dyn Error>> {
        let endpoint = format!("{}/runtimes", self.url);
        let runtimes = self
            .client
            .get(endpoint)
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<Vec<Runtime>>()
            .await?;

        Ok(runtimes)
    }

    pub async fn execute(&self, executor: &Executor) -> Result<ExecutorResponse, Box<dyn Error>> {
        let endpoint = format!("{}/execute", self.url);
        let result = self
            .client
            .post(endpoint)
            .headers(self.headers.clone())
            .json::<Executor>(executor)
            .send()
            .await?
            .json::<ExecutorResponse>()
            .await?;

        Ok(result)
    }
}
