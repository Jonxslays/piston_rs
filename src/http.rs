use std::error::Error;

use reqwest::header::{HeaderMap, HeaderValue};

use super::Executor;
use super::ExecutorResponse;
use super::Language;

#[derive(Debug)]
pub struct HttpHandler {
    url: String,
    client: reqwest::Client,
    headers: HeaderMap,
}

impl HttpHandler {
    pub fn new(key: Option<&str>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "User-Agent",
            HeaderValue::from_str("piston-rs-client").unwrap(),
        );
        headers.insert("Accept", HeaderValue::from_str("application/json").unwrap());

        if let Some(k) = key {
            headers.insert("Authorization", HeaderValue::from_str(k).unwrap());
        }

        Self {
            url: "https://emkc.org/api/v2/piston".to_string(),
            client: reqwest::Client::new(),
            headers,
        }
    }

    pub async fn fetch_languages(&self) -> Result<Vec<Language>, Box<dyn Error>> {
        let endpoint = format!("{}/runtimes", self.url);
        let languages = self
            .client
            .get(endpoint)
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<Vec<Language>>()
            .await?;

        Ok(languages)
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
