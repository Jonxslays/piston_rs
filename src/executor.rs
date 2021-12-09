use serde::{Deserialize, Serialize};

use super::File;

/// The result of code execution returned by Piston.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecResult {
    /// The text sent to `stdout` during execution.
    pub stdout: String,
    /// The text sent to `stderr` during execution.
    pub stderr: String,
    /// The text sent to both `stdout`, and `stderr` during execution.
    pub output: String,
    /// The exit code returned by the process.
    pub code: isize,
    /// The optional signal sent to the process. (`SIGKILL` etc)
    pub signal: Option<String>,
}

impl ExecResult {
    /// Whether or not the execution was ok.
    ///
    /// # Returns
    /// - [`bool`] - [`true`] if the execution returned a zero exit
    /// code.
    pub fn is_ok(&self) -> bool {
        self.code == 0
    }

    /// Whether or not the execution produced errors.
    ///
    /// # Returns
    /// - [`bool`] - [`true`] if the execution returned a non zero exit
    /// code.
    pub fn is_err(&self) -> bool {
        self.code != 0
    }
}

/// A response returned by Piston when executing code.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecResponse {
    /// The language that was used.
    pub language: String,
    /// The version of the language that was used.
    pub version: String,
    /// The result Piston sends detailing execution.
    pub run: ExecResult,
    /// The optional result Piston sends detailing compilation. This
    /// will be [`None`] for non-compiled languages.
    pub compile: Option<ExecResult>,
}

impl ExecResponse {
    /// Whether or not the request to Piston succeeded.
    ///
    /// # Returns
    /// - [`bool`] - [`true`] if a 200 status code was received from Piston.
    pub fn is_ok(&self) -> bool {
        match &self.compile {
            Some(c) => c.is_ok() && self.run.is_ok(),
            None => self.run.is_ok(),
        }
    }

    /// Whether or not the request to Piston failed.
    ///
    /// # Returns
    /// - [`bool`] - [`true`] if a non 200 status code was received from Piston.
    pub fn is_err(&self) -> bool {
        match &self.compile {
            Some(c) => c.is_err() || self.run.is_err(),
            None => self.run.is_err(),
        }
    }
}

/// An object containing information about the code being executed.
///
/// A convenient builder flow is provided by the methods associated with
/// the `Executor`. These consume self and return self for chained calls.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Executor {
    /// **Required** - The language to use for execution. Defaults to a
    /// new `String`.
    pub language: String,
    /// The version of the language to use for execution.
    /// Defaults to "*" (*most recent version*).
    pub version: String,
    /// **Required** - A `Vector` of `File`'s to send to Piston. The
    /// first file in the vector is considered the main file. Defaults
    /// to a new `Vector`.
    pub files: Vec<File>,
    /// The text to pass as stdin to the program. Defaults to a new
    /// `String`.
    pub stdin: String,
    /// The arguments to pass to the program. Defaults to a new
    /// `Vector`.
    pub args: Vec<String>,
    /// The maximum allowed time for compilation in milliseconds.
    /// Defaults to `10,000`.
    pub compile_timeout: isize,
    /// The maximum allowed time for execution in milliseconds. Defaults
    /// to `3,000`.
    pub run_timeout: isize,
    /// The maximum allowed memory usage for compilation in bytes.
    /// Defaults to `-1` (*no limit*).
    pub compile_memory_limit: isize,
    /// The maximum allowed memory usage for execution in bytes.
    /// Defaults to `-1` (*no limit*).
    pub run_memory_limit: isize,
}

impl Default for Executor {
    /// Creates a new executor. Alias for [`Executor::new`].
    ///
    /// # Returns
    /// - [`Executor`] - The new blank Executor.
    ///
    /// # Example
    /// ```
    /// let executor = piston_rs::Executor::default();
    ///
    /// assert_eq!(executor.language, String::new());
    /// assert_eq!(executor.version, String::from("*"));
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl Executor {
    /// Creates a new executor representing source code to be
    /// executed.
    ///
    /// Metadata regarding the source language and files will
    /// need to be added using the associated method calls, and other
    /// optional fields can be set as well.
    ///
    /// # Returns
    /// - [`Executor`] - The new blank Executor.
    ///
    /// # Example
    /// ```
    /// let executor = piston_rs::Executor::new();
    ///
    /// assert_eq!(executor.language, String::new());
    /// assert_eq!(executor.version, String::from("*"));
    /// ```
    pub fn new() -> Self {
        Self {
            language: String::new(),
            version: String::from("*"),
            files: vec![],
            stdin: String::new(),
            args: vec![],
            compile_timeout: 10000,
            run_timeout: 3000,
            compile_memory_limit: -1,
            run_memory_limit: -1,
        }
    }

    /// Resets the executor back to a `new` state, ready to be
    /// configured again and sent to Piston after metadata is added.
    /// This method mutates the existing executor in place.
    ///
    /// # Example
    /// ```
    /// let mut executor = piston_rs::Executor::new()
    ///     .set_language("rust");
    ///
    /// assert_eq!(executor.language, "rust".to_string());
    ///
    /// executor.reset();
    ///
    /// assert_eq!(executor.language, String::new());
    /// ```
    pub fn reset(&mut self) {
        self.language = String::new();
        self.version = String::from("*");
        self.files = vec![];
        self.stdin = String::new();
        self.args = vec![];
        self.compile_timeout = 10000;
        self.run_timeout = 3000;
        self.compile_memory_limit = -1;
        self.run_memory_limit = -1;
    }

    /// Sets the language to use for execution.
    ///
    /// # Arguments
    /// - `language` - The language to use.
    ///
    /// # Returns
    /// - [`Self`] - For chained method calls.
    ///
    /// # Example
    /// ```
    /// let executor = piston_rs::Executor::new()
    ///     .set_language("rust");
    ///
    /// assert_eq!(executor.language, "rust".to_string());
    /// ```
    pub fn set_language(mut self, language: &str) -> Self {
        self.language = language.to_lowercase();
        self
    }

    /// Sets the version of the language to use for execution.
    ///
    /// # Arguments
    /// - `version` - The version to use.
    ///
    /// # Returns
    /// - [`Self`] - For chained method calls.
    ///
    /// # Example
    /// ```
    /// let executor = piston_rs::Executor::new()
    ///     .set_version("1.50.0");
    ///
    /// assert_eq!(executor.version, "1.50.0".to_string());
    /// ```
    pub fn set_version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    /// Adds a [`File`] containing the code to be executed. Does not
    /// overwrite any existing files.
    ///
    /// # Arguments
    /// - `file` - The file to add.
    ///
    /// # Returns
    /// - [`Self`] - For chained method calls.
    ///
    /// # Example
    /// ```
    /// let file = piston_rs::File::default();
    ///
    /// let executor = piston_rs::Executor::new()
    ///     .add_file(file.clone());
    ///
    /// assert_eq!(executor.files, [file].to_vec());
    /// ```
    pub fn add_file(mut self, file: File) -> Self {
        self.files.push(file);
        self
    }

    /// Adds multiple [`File`]'s containing the code to be executed.
    /// Does not overwrite any existing files.
    ///
    /// # Arguments
    /// - `files` - The files to add.
    ///
    /// # Returns
    /// - [`Self`] - For chained method calls.
    ///
    /// # Example
    /// ```
    /// let mut files = vec![];
    ///
    /// for _ in 0..3 {
    ///     files.push(piston_rs::File::default());
    /// }
    ///
    /// let executor = piston_rs::Executor::new()
    ///     .add_files(files.clone());
    ///
    /// assert_eq!(executor.files, files);
    /// ```
    pub fn add_files(mut self, files: Vec<File>) -> Self {
        self.files.extend(files);
        self
    }

    /// Adds multiple [`File`]'s containing the code to be executed.
    /// Overwrites any existing files. This method mutates the existing
    /// executor in place. **Overwrites any existing files.**
    ///
    /// # Arguments
    /// - `files` - The files to replace existing files with.
    ///
    /// # Example
    /// ```
    /// let old_file = piston_rs::File::default()
    ///     .set_name("old_file.rs");
    ///
    /// let mut executor = piston_rs::Executor::new()
    ///     .add_file(old_file.clone());
    ///
    /// assert_eq!(executor.files.len(), 1);
    /// assert_eq!(executor.files[0].name, "old_file.rs".to_string());
    ///
    /// let new_files = vec![
    ///     piston_rs::File::default().set_name("new_file1.rs"),
    ///     piston_rs::File::default().set_name("new_file2.rs"),
    /// ];
    ///
    /// executor.set_files(new_files.clone());
    ///
    /// assert_eq!(executor.files.len(), 2);
    /// assert_eq!(executor.files[0].name, "new_file1.rs".to_string());
    /// assert_eq!(executor.files[1].name, "new_file2.rs".to_string());
    /// ```
    pub fn set_files(&mut self, files: Vec<File>) {
        self.files = files;
    }

    /// Sets the text to pass as `stdin` to the program.
    ///
    /// # Arguments
    /// - `stdin` - The text to set.
    ///
    /// # Returns
    /// - [`Self`] - For chained method calls.
    ///
    /// # Example
    /// ```
    /// let executor = piston_rs::Executor::new()
    ///     .set_stdin("Fearless concurrency");
    ///
    /// assert_eq!(executor.stdin, "Fearless concurrency".to_string());
    /// ```
    pub fn set_stdin(mut self, stdin: &str) -> Self {
        self.stdin = stdin.to_string();
        self
    }

    /// Adds an arg to be passed as a command line argument. Does not
    /// overwrite any existing args.
    ///
    /// # Arguments
    /// - `arg` - The arg to add.
    ///
    /// # Returns
    /// - [`Self`] - For chained method calls.
    ///
    /// # Example
    /// ```
    /// let executor = piston_rs::Executor::new()
    ///     .add_arg("--verbose");
    ///
    /// assert_eq!(executor.args, vec!["--verbose".to_string()]);
    /// ```
    pub fn add_arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    /// Adds multiple args to be passed as a command line arguments.
    /// Does not overwrite any existing args.
    ///
    /// # Arguments
    /// - `args` - The args to add.
    ///
    /// # Example
    /// ```
    /// let executor = piston_rs::Executor::new()
    ///     .add_args(vec!["--verbose"]);
    ///
    /// assert_eq!(executor.args, vec!["--verbose".to_string()]);
    /// ```
    pub fn add_args(mut self, args: Vec<&str>) -> Self {
        self.args.extend(args.iter().map(|a| a.to_string()));
        self
    }

    /// Adds multiple args to be passed as a command line arguments.
    /// Overwrites any existing args. This method mutates the existing
    /// executor in place. **Overwrites any existing args.**
    ///
    /// # Arguments
    /// - `args` - The args to replace existing args with.
    ///
    /// # Example
    /// ```
    /// let mut executor = piston_rs::Executor::new()
    ///     .add_arg("--verbose");
    ///
    /// assert_eq!(executor.args.len(), 1);
    /// assert_eq!(executor.args[0], "--verbose".to_string());
    ///
    /// let args = vec!["commit", "-S"];
    /// executor.set_args(args);
    ///
    /// assert_eq!(executor.args.len(), 2);
    /// assert_eq!(executor.args[0], "commit".to_string());
    /// assert_eq!(executor.args[1], "-S".to_string());
    /// ```
    pub fn set_args(&mut self, args: Vec<&str>) {
        self.args = args.iter().map(|a| a.to_string()).collect();
    }

    /// Sets the maximum allowed time for compilation in milliseconds.
    ///
    /// # Arguments
    /// - `timeout` - The timeout to set.
    ///
    /// # Returns
    /// - [`Self`] - For chained method calls.
    ///
    /// # Example
    /// ```
    /// let executor = piston_rs::Executor::new()
    ///     .set_compile_timeout(5000);
    ///
    /// assert_eq!(executor.compile_timeout, 5000);
    /// ```
    pub fn set_compile_timeout(mut self, timeout: isize) -> Self {
        self.compile_timeout = timeout;
        self
    }

    /// Sets the maximum allowed time for execution in milliseconds.
    ///
    /// # Arguments
    /// - `timeout` - The timeout to set.
    ///
    /// # Returns
    /// - [`Self`] - For chained method calls.
    ///
    /// # Example
    /// ```
    /// let executor = piston_rs::Executor::new()
    ///     .set_run_timeout(1500);
    ///
    /// assert_eq!(executor.run_timeout, 1500);
    /// ```
    pub fn set_run_timeout(mut self, timeout: isize) -> Self {
        self.run_timeout = timeout;
        self
    }

    /// Sets the maximum allowed memory usage for compilation in bytes.
    ///
    /// # Arguments
    /// - `limit` - The memory limit to set.
    ///
    /// # Returns
    /// - [`Self`] - For chained method calls.
    ///
    /// # Example
    /// ```
    /// let executor = piston_rs::Executor::new()
    ///     .set_compile_memory_limit(100_000_000);
    ///
    /// assert_eq!(executor.compile_memory_limit, 100_000_000);
    /// ```
    pub fn set_compile_memory_limit(mut self, limit: isize) -> Self {
        self.compile_memory_limit = limit;
        self
    }

    /// Sets the maximum allowed memory usage for execution in bytes.
    ///
    /// # Arguments
    /// - `limit` - The memory limit to set.
    ///
    /// # Returns
    /// - [`Self`] - For chained method calls.
    ///
    /// # Example
    /// ```
    /// let executor = piston_rs::Executor::new()
    ///     .set_run_memory_limit(100_000_000);
    ///
    /// assert_eq!(executor.run_memory_limit, 100_000_000);
    /// ```
    pub fn set_run_memory_limit(mut self, limit: isize) -> Self {
        self.run_memory_limit = limit;
        self
    }
}

#[cfg(test)]
mod test_execution_result {
    use super::ExecResult;

    fn generate_result(stdout: &str, stderr: &str, code: isize) -> ExecResult {
        ExecResult {
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            output: format!("{}\n{}", stdout, stderr),
            code,
            signal: None,
        }
    }

    #[test]
    fn test_is_ok() {
        let result = generate_result("Hello, world", "", 0);

        assert!(result.is_ok());
        assert!(!result.is_err());
    }

    #[test]
    fn test_is_err() {
        let result = generate_result("", "Error!", 1);

        assert!(!result.is_ok());
        assert!(result.is_err());
    }

    #[test]
    fn test_is_err_with_stdout() {
        let result = generate_result("Hello, world", "Error!", 1);

        assert!(!result.is_ok());
        assert!(result.is_err());
    }
}
