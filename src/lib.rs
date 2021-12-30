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
//!         if let Some(c) = response.compile {
//!             println!("Compilation: {}", c.output);
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
use std::fs;
use std::path::{Path, PathBuf};

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

/// The result from attempting to load a [`File`].
type LoadResult<T> = Result<T, LoadError>;

/// The error that is returned when loading from a [`File`] on disk
/// fails for any reason.
#[derive(Debug, Clone)]
pub struct LoadError {
    /// The details of this error.
    pub details: String,
}

impl LoadError {
    /// Generates a new [`LoadError`].
    ///
    /// # Arguments
    /// - `details` - The details of the error.
    ///
    /// # Returns
    /// - [`LoadError`] - The new error.
    ///
    /// # Examples
    /// ```
    /// let e = piston_rs::LoadError::new("err");
    ///
    /// assert_eq!(e.details, "err".to_string())
    /// ```
    pub fn new(details: &str) -> Self {
        Self {
            details: details.into(),
        }
    }
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
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
            name: String::new(),
            content: String::new(),
            encoding: String::from("utf8"),
        }
    }
}

impl File {
    /// Creates a new [`File`].
    ///
    /// # Arguments
    /// - `name` - The name to use.
    /// - `content` - The content to use.
    /// - `encoding` - The encoding to use. Must be one of "utf8",
    /// "hex", or "base64".
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

    /// Creates a new [`File`] from an existing file on disk.
    ///
    /// # Arguments
    /// - `path` - The path to the file.
    ///
    /// # Returns
    /// - [`File`] - The new File.
    ///
    /// # Example
    /// ```
    /// let file = piston_rs::File::load_from("src/lib.rs").unwrap();
    ///
    /// assert!(file.content.contains("pub fn load_from"));
    /// assert_eq!(file.name, "lib.rs".to_string());
    /// assert_eq!(file.encoding, "utf8".to_string());
    /// ```
    pub fn load_from(path: &str) -> LoadResult<Self> {
        let path = PathBuf::from(path);

        if !path.is_file() {
            return Err(LoadError::new("File does not exist, or is a directory"));
        }

        let name = match path.file_name() {
            Some(n) => n.to_string_lossy(),
            None => {
                return Err(LoadError::new("Unable to parse file name"));
            }
        };

        Ok(Self {
            name: name.to_string(),
            content: File::load_contents(&path)?,
            encoding: String::from("utf8"),
        })
    }

    fn load_contents(path: &Path) -> LoadResult<String> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(content),
            Err(e) => Err(LoadError::new(&e.to_string())),
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
    #[must_use]
    pub fn set_content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    /// Sets the content of the file to the contents of an existing
    /// file on disk.
    ///
    /// # Arguments
    /// - `path` - The path to the file.
    ///
    /// # Returns
    /// - [`Self`] - For chained method calls.
    ///
    /// # Example
    /// ```
    /// let file = piston_rs::File::default()
    ///     .load_contents_from("src/lib.rs");
    ///
    /// assert!(file.is_ok());
    /// assert!(file.unwrap().content.contains("pub fn load_contents_from"));
    /// ```
    pub fn load_content_from(mut self, path: &str) -> LoadResult<Self> {
        let path = PathBuf::from(path);
        self.content = File::load_contents(&path)?;
        Ok(self)
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
    #[must_use]
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
    #[must_use]
    pub fn set_encoding(mut self, encoding: &str) -> Self {
        self.encoding = encoding.to_string();
        self
    }
}

#[cfg(test)]
mod test_file_private {
    use super::File;
    use super::Runtime;
    use std::path::PathBuf;

    #[test]
    fn test_load_contents() {
        let path = PathBuf::from(file!());
        let contents = File::load_contents(&path).unwrap();

        assert!(contents.contains("mod test_file_private {"));
    }

    #[test]
    fn test_load_contents_non_existent() {
        let path = PathBuf::from("/path/doesnt/exist");
        let contents = File::load_contents(&path);

        assert!(contents.is_err());
        let err = contents.unwrap_err();

        assert!(err.details.contains("No such file"));
        assert!(format!("{}", err).contains("No such file"));

        let err2 = err.clone();
        assert_eq!(err.details, err2.details);
    }

    #[test]
    fn test_runtime_creation() {
        let rt = Runtime {
            language: "clojure".to_string(),
            version: "9000".to_string(),
            aliases: vec![],
        };

        let rt2 = rt.clone();
        assert_eq!(rt.language, rt2.language);
        assert_eq!(rt.version, rt2.version);
        assert_eq!(rt.aliases, rt2.aliases);

        assert_eq!(rt.language, "clojure".to_string());
        assert_eq!(rt.version, "9000".to_string());
        assert!(rt.aliases.is_empty());
    }
}
