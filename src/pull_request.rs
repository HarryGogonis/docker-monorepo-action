use serde::{Deserialize, Serialize};
use std::fs::File;
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

pub fn new(file_path: &str) -> Result<PullRequest, String> {
    let mut buffer = String::new();

    let mut file = match File::open(file_path) {
        Err(e) => return Err(format!("Failed to read file {}. Reason: {}", file_path, e)),
        Ok(f) => f,
    };

    let _ = file.read_to_string(&mut buffer);

    match serde_json::from_str(&buffer) {
        Ok(v) => Ok(v),
        Err(e) => return Err(format!("Failed to parse json: {}", e)),
    }
}
