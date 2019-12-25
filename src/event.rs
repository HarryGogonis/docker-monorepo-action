mod pull_request;
mod push;
mod utils;
use crate::repo::Diffable;
use pull_request::PullRequest;
use push::Push;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum EventError {
    Io(std::io::Error, String),
    JSON(serde_json::error::Error),
    EventType(String),
}

pub struct Event {
    pub payload: EventPayload,
    pub path: String,
}

pub type EventPayload = Box<dyn Diffable>;

pub trait EventParser {
    fn read(json: &str) -> Result<EventPayload, EventError>;
}

pub fn read(
    event_path: String,
    event_name: String,
    workflow_path: String,
) -> Result<Event, EventError> {
    let json = match utils::read_file(&event_path) {
        Err(e) => return Err(EventError::Io(e, event_path)),
        Ok(f) => f,
    };

    let payload = match event_name.as_str() {
        "pull_request" => PullRequest::read(&json)?,
        "push" => Push::read(&json)?,
        _ => return Err(EventError::EventType(event_name)),
    };

    Ok(Event {
        payload,
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
            EventError::EventType(ref e) => write!(f, "Event type {} not supported", e),
        }
    }
}

impl error::Error for EventError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            EventError::Io(ref e, _) => Some(e),
            EventError::JSON(ref e) => Some(e),
            EventError::EventType(_) => None,
        }
    }
}
