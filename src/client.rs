use std::error::Error;

use super::http::HttpHandler;
use super::Executor;
use super::ExecutorResponse;
use super::Language;

#[derive(Debug)]
pub struct Client {
    http: HttpHandler,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            http: HttpHandler::new(None),
        }
    }

    pub fn new_with_key(key: &str) -> Self {
        Self {
            http: HttpHandler::new(Some(key)),
        }
    }

    pub async fn fetch_languages(&self) -> Result<Vec<Language>, Box<dyn Error>> {
        self.http.fetch_languages().await
    }

    pub async fn execute(&self, executor: &Executor) -> Result<ExecutorResponse, Box<dyn Error>> {
        self.http.execute(executor).await
    }
}
