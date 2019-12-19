use serde::{Deserialize, Serialize};
use std::error::Error;
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

pub fn read_file(filePath: &str) -> Result<PullRequest, String> {
    let mut buffer = String::new();

    let mut file = match File::open(filePath) {
        Err(e) => return Err(format!("Failed to read file {}. Reason: {}", filePath, e)),
        Ok(f) => f,
    };

    file.read_to_string(&mut buffer);

    match serde_json::from_str(&buffer) {
        Ok(v) => Ok(v),
        Err(e) => return Err(format!("Failed to parse json: {}", e)),
    }
}
