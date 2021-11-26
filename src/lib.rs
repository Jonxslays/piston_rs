use serde::{Deserialize, Serialize};

mod client;
mod executor;

pub use client::Client;
pub use executor::Executor;
pub use executor::ExecutorResponse;
pub use executor::ExecutionResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Runtime {
    pub language: String,
    pub version: String,
    pub aliases: Vec<String>,
}

impl Runtime {
    pub fn new(language: String, version: String, aliases: Vec<String>) -> Self {
        Self {
            language,
            version,
            aliases,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub content: String,
    pub encoding: String,
}

impl Default for File {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            content: "".to_string(),
            encoding: "utf8".to_string(),
        }
    }
}

impl File {
    pub fn new(name: &str, content: &str, encoding: &str) -> Self {
        Self {
            name: name.to_string(),
            content: content.to_string(),
            encoding: encoding.to_string(),
        }
    }

    pub fn set_content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    pub fn set_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn set_encoding(mut self, encoding: &str) -> Self {
        self.encoding = encoding.to_string();
        self
    }
}
