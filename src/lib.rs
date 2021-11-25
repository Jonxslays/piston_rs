use serde::{Deserialize, Serialize};

mod client;
mod http;
pub use client::Client;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Language {
    #[serde(rename = "language")]
    pub name: String,
    pub version: String,
    pub aliases: Vec<String>,
}

impl Language {
    pub fn new(name: String, version: String, aliases: Vec<String>) -> Self {
        Self {
            name,
            version,
            aliases,
        }
    }
}
