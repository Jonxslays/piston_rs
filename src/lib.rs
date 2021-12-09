//! `piston_rs` - An async wrapper for the
//! [Piston](https://github.com/engineer-man/piston) code execution
//! engine.
//!
//! Aiming to make interacting with Piston fun and easy.
//!
//! ## Getting started
//!
//! Check out the [`Client`] and [`Executor`] documentation.
//!
//! ##### Make requests to Piston
//!
//! ```
//! # #[tokio::test]
//! # async fn example() {
//! let client = piston_rs::Client::new();
//! let executor = piston_rs::Executor::new()
//!     .set_language("rust")
//!     .set_version("*")
//!     .add_file(
//!         piston_rs::File::default()
//!             .set_name("main.rs")
//!             .set_content("fn main() { println!(\"42\"); }")
//!     );
//!
//! match client.execute(&executor).await {
//!     Ok(response) => {
//!         println!("Language: {}", response.language);
//!         println!("Version: {}", response.version);
//!
//!         if response.is_err() {
//!             if let Some(c) = response.compile {
//!                 println!("Compilation: {}", c.output);
//!             }
//!         }
//!
//!         println!("Output: {}", response.run.output);
//!     }
//!     Err(e) => {
//!         println!("Something went wrong contacting Piston.");
//!         println!("{}", e);
//!     }
//! }
//! # }
//! ```

// RIP shrimpie, gone but not forgotten.

use serde::{Deserialize, Serialize};

mod client;
mod executor;

pub use client::Client;
pub use executor::ExecResponse;
pub use executor::ExecResult;
pub use executor::Executor;

/// A runtime available to be used by Piston.
///
/// ##### Note
///
/// Runtimes are not meant to be created manually. Instead, they should
/// be fetched from Piston using [`Client::fetch_runtimes`] and stored,
/// if you have a need for the information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Runtime {
    /// The language.
    pub language: String,
    /// The version of the language.
    pub version: String,
    /// The aliases associated with this runtime.
    pub aliases: Vec<String>,
}

/// A file that contains source code to be executed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct File {
    // The name of the file. Defaults to a new `String`.
    pub name: String,
    /// **Required** The content of the file.
    pub content: String,
    /// The encoding of the file. Defaults to "utf8".
    pub encoding: String,
}

impl Default for File {
    /// Creates an unnamed new [`File`] with utf8 encoding and no
    /// content.
    ///
    /// # Returns
    /// - [`File`] - The new blank File.
    ///
    /// # Example
    /// ```
    /// let file = piston_rs::File::default();
    ///
    /// assert_eq!(file.name, String::new());
    /// assert_eq!(file.content, String::new());
    /// assert_eq!(file.encoding, "utf8".to_string());
    /// ```
    fn default() -> Self {
        Self {
            name: "".to_string(),
            content: "".to_string(),
            encoding: "utf8".to_string(),
        }
    }
}

impl File {
    /// Creates a new [`File`].
    ///
    /// # Returns
    /// - [`File`] - The new File.
    ///
    /// # Example
    /// ```
    /// let file = piston_rs::File::new(
    ///     "script.sh",
    ///     "ZWNobyBIZWxsbywgV29ybGQh",
    ///     "base64",
    /// );
    ///
    /// assert!(file.content.contains("ZWNobyBIZWxsbywgV29ybGQh"));
    /// assert_eq!(file.name, "script.sh".to_string());
    /// assert_eq!(file.encoding, "base64".to_string());
    /// ```
    pub fn new(name: &str, content: &str, encoding: &str) -> Self {
        Self {
            name: name.to_string(),
            content: content.to_string(),
            encoding: encoding.to_string(),
        }
    }

    /// Sets the content of the file.
    ///
    /// # Arguments
    /// - `content` - The content to use.
    ///
    /// # Returns
    /// - [`Self`] - For chained method calls.
    ///
    /// # Example
    /// ```
    /// let file = piston_rs::File::default()
    ///     .set_content("print(\"Hello, world!\")");
    ///
    /// assert_eq!(file.content, "print(\"Hello, world!\")".to_string());
    /// ```
    pub fn set_content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    /// Sets the name of the file.
    ///
    /// # Arguments
    /// - `name` - The name to use.
    ///
    /// # Returns
    /// - [`Self`] - For chained method calls.
    ///
    /// # Example
    /// ```
    /// let file = piston_rs::File::default()
    ///     .set_name("__main__.py");
    ///
    /// assert_eq!(file.name, "__main__.py".to_string());
    /// ```
    pub fn set_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Sets the encoding of the file.
    ///
    /// # Arguments
    /// - `encoding` - The encoding to use. Must be one of "utf8",
    /// "hex", or "base64".
    ///
    /// # Returns
    /// - [`Self`] - For chained method calls.
    ///
    /// # Example
    /// ```
    /// let file = piston_rs::File::default()
    ///     .set_encoding("hex");
    ///
    /// assert_eq!(file.encoding, "hex".to_string());
    /// ```
    pub fn set_encoding(mut self, encoding: &str) -> Self {
        self.encoding = encoding.to_string();
        self
    }
}
