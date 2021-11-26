use serde::{Deserialize, Serialize};

use super::File;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub output: String,
    pub code: isize,
    pub signal: Option<String>,
}

impl ExecutionResult {
    pub fn is_ok(&self) -> bool {
        !self.stdout.is_empty() && self.stderr.is_empty()
    }

    pub fn is_err(&self) -> bool {
        self.stdout.is_empty() && !self.stderr.is_empty()
    }

    pub fn is_ok_with_err(&self) -> bool {
        !self.stdout.is_empty() && !self.stderr.is_empty()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutorResponse {
    pub language: String,
    pub version: String,
    pub run: ExecutionResult,
    pub compile: Option<ExecutionResult>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Executor {
    language: String,
    version: String,
    files: Vec<File>,
    stdin: String,
    args: Vec<String>,
    compile_timeout: isize,
    run_timeout: isize,
    compile_memory_limit: isize,
    run_memory_limit: isize,
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor {
    pub fn new() -> Self {
        Self {
            language: String::new(),
            version: String::new(),
            files: vec![],
            stdin: String::new(),
            args: vec![],
            compile_timeout: 10000,
            run_timeout: 3000,
            compile_memory_limit: -1,
            run_memory_limit: -1,
        }
    }

    pub fn set_language(mut self, language: &str) -> Self {
        self.language = language.to_lowercase();
        self
    }

    pub fn set_version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    pub fn add_file(mut self, file: File) -> Self {
        self.files.push(file);
        self
    }

    pub fn add_files(mut self, files: Vec<File>) -> Self {
        self.files.extend(files);
        self
    }

    pub fn set_files(mut self, files: Vec<File>) -> Self {
        self.files = files;
        self
    }

    pub fn set_stdin(mut self, stdin: &str) -> Self {
        self.stdin = stdin.to_string();
        self
    }

    pub fn add_arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    pub fn add_args(mut self, args: Vec<&str>) -> Self {
        self.args.extend(args.iter().map(|a| a.to_string()));
        self
    }

    pub fn set_args(mut self, args: Vec<&str>) -> Self {
        self.args = args.iter().map(|a| a.to_string()).collect();
        self
    }

    pub fn set_compile_timeout(mut self, timeout: isize) -> Self {
        self.compile_timeout = timeout;
        self
    }

    pub fn set_run_timeout(mut self, timeout: isize) -> Self {
        self.run_timeout = timeout;
        self
    }

    pub fn set_compile_memory_limit(mut self, limit: isize) -> Self {
        self.compile_memory_limit = limit;
        self
    }

    pub fn set_run_memory_limit(mut self, limit: isize) -> Self {
        self.run_memory_limit = limit;
        self
    }
}
