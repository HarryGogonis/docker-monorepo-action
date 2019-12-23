use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct GitRef {
    pub r#ref: String,
    pub sha: String,
}

#[derive(Serialize, Deserialize)]
pub struct PullRequest {
    pub base: GitRef,
    pub head: GitRef,
}

type Result = std::result::Result<PullRequest, PullRequestError>;

#[derive(Debug)]
pub enum PullRequestError {
    Io(io::Error, String),
    JSON(serde_json::error::Error),
}

impl PullRequest {
    pub fn read(file_path: &str) -> Result {
        let mut buffer = String::new();

        let mut file = match File::open(file_path) {
            Err(e) => return Err(PullRequestError::Io(e, String::from(file_path))),
            Ok(f) => f,
        };

        let _ = file.read_to_string(&mut buffer);

        match serde_json::from_str(&buffer) {
            Ok(v) => Ok(v),
            Err(e) => return Err(PullRequestError::JSON(e)),
        }
    }
}

impl fmt::Display for PullRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PullRequestError::Io(ref e, ref path) => write!(f, "Failed to read file: '{}':\n\t {}", path, e),
            PullRequestError::JSON(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for PullRequestError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            PullRequestError::Io(ref e, _) => Some(e),
            PullRequestError::JSON(ref e) => Some(e),
        }
    }
}
