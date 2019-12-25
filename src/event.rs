use crate::pull_request::PullRequest;
use crate::repo::Diffable;
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

#[derive(Debug)]
pub enum EventError {
    Io(io::Error, String),
    JSON(serde_json::error::Error),
}

pub struct Event {
    pub payload: Box<dyn Diffable>,
    pub path: String,
}

pub fn read(event_path: String, workflow_path: String) -> Result<Event, EventError> {
    let mut buffer = String::new();

    let mut file = match File::open(&event_path) {
        Err(e) => return Err(EventError::Io(e, event_path)),
        Ok(f) => f,
    };

    let _ = file.read_to_string(&mut buffer);

    // TODO support multiple event types
    let pr = PullRequest::read(buffer)?;
    Ok(Event {
        payload: Box::new(pr),
        path: workflow_path,
    })
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EventError::Io(ref e, ref path) => {
                write!(f, "Failed to read file: '{}':\n\t {}", path, e)
            }
            EventError::JSON(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for EventError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            EventError::Io(ref e, _) => Some(e),
            EventError::JSON(ref e) => Some(e),
        }
    }
}
