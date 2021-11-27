use std::error::Error;

use reqwest::header::{HeaderMap, HeaderValue};

use super::ExecResponse;
use super::ExecResult;
use super::Executor;
use super::Runtime;

/// A client used to send requests to Piston.
#[derive(Debug)]
pub struct Client {
    /// The base url for Piston.
    url: String,
    /// The reqwest client to use.
    client: reqwest::Client,
    /// The headers to send with each request.
    headers: HeaderMap,
}

impl Default for Client {
    /// Creates a new client. Alias for [`Client::new`].
    ///
    /// # Returns
    /// - [`Client`] - The new Client.
    ///
    /// # Example
    /// ```
    /// let client = piston_rs::Client::default();
    ///
    /// assert!(client.get_headers().contains_key("Accept"));
    /// assert!(client.get_headers().contains_key("User-Agent"));
    /// assert!(!client.get_headers().contains_key("Authorization"));
    /// assert_eq!(client.get_url(), "https://emkc.org/api/v2/piston".to_string());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// Creates a new client.
    ///
    /// # Returns
    /// - [`Client`] - The new Client.
    ///
    /// # Example
    /// ```
    /// let client = piston_rs::Client::new();
    ///
    /// assert!(client.get_headers().contains_key("Accept"));
    /// assert!(client.get_headers().contains_key("User-Agent"));
    /// assert!(!client.get_headers().contains_key("Authorization"));
    /// ```
    pub fn new() -> Self {
        Self {
            url: "https://emkc.org/api/v2/piston".to_string(),
            client: reqwest::Client::new(),
            headers: Self::generate_headers(None),
        }
    }

    /// Creates a new client, with an api key.
    ///
    /// # Returns
    /// - [`Client`] - The new Client.
    ///
    /// # Example
    /// ```
    /// let client = piston_rs::Client::with_key("123abc");
    ///
    /// assert!(client.get_headers().contains_key("Authorization"));
    /// assert_eq!(client.get_headers().get("Authorization").unwrap(), "123abc");
    /// ```
    pub fn with_key(key: &str) -> Self {
        Self {
            url: "https://emkc.org/api/v2/piston".to_string(),
            client: reqwest::Client::new(),
            headers: Self::generate_headers(Some(key)),
        }
    }

    /// The base url for the Piston V2 API.
    ///
    /// # Returns
    ///
    /// - [`String`] - The requested url.
    ///
    /// # Example
    /// ```
    /// let client = piston_rs::Client::new();
    ///
    /// assert_eq!(client.get_url(), "https://emkc.org/api/v2/piston".to_string());
    /// ```
    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    /// The headers being used by this client.
    ///
    /// # Returns
    ///
    /// - [`HeaderMap`] - A map of Header key, value pairs.
    ///
    /// # Example
    /// ```
    /// let client = piston_rs::Client::new();
    /// let headers = client.get_headers();
    ///
    /// assert_eq!(headers.get("Accept").unwrap(), "application/json");
    /// ```
    pub fn get_headers(&self) -> HeaderMap {
        self.headers.clone()
    }

    /// Generates the headers the client should use.
    ///
    /// # Returns
    ///
    /// - [`HeaderMap`] - A map of Header key, value pairs.
    ///
    /// # Example
    /// ```ignore # Fails to compile (private function)
    /// let headers = piston_rs::Client::generate_headers(None);
    ///
    /// assert!(!headers.contains_key("Authorization"));
    /// assert_eq!(headers.get("Accept").unwrap(), "application/json");
    /// assert_eq!(headers.get("User-Agent").unwrap(), "piston-rs");
    ///
    /// let headers = piston_rs::Client::generate_headers(Some("123abc"));
    ///
    /// assert_eq!(headers.get("Authorization").unwrap(), "123abc");
    /// assert_eq!(headers.get("Accept").unwrap(), "application/json");
    /// assert_eq!(headers.get("User-Agent").unwrap(), "piston-rs");
    /// ```
    fn generate_headers(key: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::with_capacity(3);
        headers.insert("Accept", HeaderValue::from_str("application/json").unwrap());
        headers.insert("User-Agent", HeaderValue::from_str("piston-rs").unwrap());

        if let Some(k) = key {
            headers.insert("Authorization", HeaderValue::from_str(k).unwrap());
        };

        headers
    }

    /// Fetches the runtimes from Piston. **This is an http request**.
    ///
    /// # Returns
    /// - [`Result<Vec<Runtime>, Box<dyn Error>>`] - The available
    /// runtimes or the error, if any.
    ///
    /// # Example
    /// ```no_run
    /// # #[tokio::test]
    /// # async fn test_fetch_runtimes() {
    /// let client = piston_rs::Client::new();
    ///
    /// if let Ok(runtimes) = client.fetch_runtimes().await {
    ///     assert!(!runtimes.is_empty());
    /// } else {
    ///     // There was an error contacting Piston.
    /// }
    /// # }
    /// ```
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

    /// Executes code using a given executor. **This is an http
    /// request**.
    ///
    /// # Returns
    /// - [`Result<ExecutorResponse, Box<dyn Error>>`] - The response
    /// from Piston or the error, if any.
    ///
    /// # Example
    /// ```no_run
    /// # #[tokio::test]
    /// # async fn test_execute() {
    /// let client = piston_rs::Client::new();
    /// let executor = piston_rs::Executor::new()
    ///     .set_language("rust")
    ///     .set_version("1.50.0")
    ///     .add_file(piston_rs::File::default().set_content(
    ///         "fn main() { println!(\"42\"); }",
    ///     ));
    ///
    /// if let Ok(response) = client.execute(&executor).await {
    ///     assert!(response.compile.is_some());
    ///     assert!(response.run.is_ok());
    ///     assert!(response.is_ok());
    /// } else {
    ///     // There was an error contacting Piston.
    /// }
    /// # }
    /// ```
    pub async fn execute(&self, executor: &Executor) -> Result<ExecResponse, Box<dyn Error>> {
        let endpoint = format!("{}/execute", self.url);

        match self
            .client
            .post(endpoint)
            .headers(self.headers.clone())
            .json::<Executor>(executor)
            .send()
            .await
        {
            Ok(data) => match data.status() {
                reqwest::StatusCode::OK => Ok(data.json::<ExecResponse>().await?),
                _ => {
                    let exec_result = ExecResult {
                        stdout: String::new(),
                        stderr: String::new(),
                        output: String::new(),
                        code: 0,
                        signal: None,
                    };

                    let exec_response = ExecResponse {
                        language: String::new(),
                        version: String::new(),
                        run: exec_result.clone(),
                        compile: None,
                        message: Some(format!("{}: {}", data.status(), data.text().await?)),
                    };

                    Ok(exec_response)
                }
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}

#[cfg(test)]
mod test_client_private {
    use super::Client;

    #[test]
    fn test_gen_headers_no_key() {
        let headers = Client::generate_headers(None);

        assert!(!headers.contains_key("Authorization"));
        assert_eq!(headers.get("Accept").unwrap(), "application/json");
        assert_eq!(headers.get("User-Agent").unwrap(), "piston-rs");
    }

    #[test]
    fn test_gen_headers_with_key() {
        let headers = Client::generate_headers(Some("123abc"));

        assert_eq!(headers.get("Authorization").unwrap(), "123abc");
        assert_eq!(headers.get("Accept").unwrap(), "application/json");
        assert_eq!(headers.get("User-Agent").unwrap(), "piston-rs");
    }
}
