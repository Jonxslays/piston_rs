use std::error::Error;

use super::http::HttpHandler;
use super::Executor;
use super::ExecutorResponse;
use super::Language;

#[derive(Debug)]
pub struct Client {
    http: HttpHandler,
    languages: Vec<Language>,
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
            languages: vec![],
        }
    }

    pub fn new_with_key(key: &str) -> Self {
        Self {
            http: HttpHandler::new(Some(key)),
            languages: vec![],
        }
    }

    pub async fn fetch_languages(&mut self) -> Result<&Vec<Language>, Box<dyn Error>> {
        if !self.languages.len() == 0 {
            self.languages.clear();
        }

        self.languages.extend(self.http.fetch_languages().await?);
        Ok(&self.languages)
    }

    pub fn get_languages(&self) -> Vec<Language> {
        self.languages.clone()
    }

    pub async fn execute(&self, executor: &Executor) -> Result<ExecutorResponse, Box<dyn Error>> {
        self.http.execute(executor).await
    }
}
