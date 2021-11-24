use std::cell::RefCell;
use std::error::Error;

use super::http::HttpHandler;
use super::Language;

#[derive(Debug)]
pub struct Client {
    http: HttpHandler,
    pub languages: RefCell<Vec<Language>>,
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
            languages: RefCell::new(vec![]),
        }
    }

    pub fn new_with_key(key: &str) -> Self {
        Self {
            http: HttpHandler::new(Some(key)),
            languages: RefCell::new(vec![]),
        }
    }

    pub async fn get_languages(&self) -> Result<(), Box<dyn Error>> {
        let languages = self.http.get_languages().await?;
        self.languages.borrow_mut().extend(languages);
        Ok(())
    }
}
