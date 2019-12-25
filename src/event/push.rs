use crate::event::{EventError, EventParser, EventPayload};
use crate::repo::Diffable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Push {
    pub before: String,
    pub after: String,
}

impl Diffable for Push {
    fn get_commit_range(&self) -> (String, String) {
        let base_commit = self.before.clone();
        let head_commit = self.after.clone();
        (base_commit, head_commit)
    }
}

impl EventParser for Push {
    fn read(json: &str) -> Result<EventPayload, EventError> {
        let v: Push = match serde_json::from_str(json) {
            Ok(pr) => pr,
            Err(e) => return Err(EventError::JSON(e)),
        };
        Ok(Box::new(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::utils;

    #[test]
    fn test_read() {
        let json = utils::read_file("./fixtures/push.json").unwrap();

        let pr = Push::read(&json).unwrap();

        let expected = (
            String::from("c467a407711867b6b9a2df8bf54c548587ff54ed"),
            String::from("a08ade107ca1d97f80c42c2e44abc7a38d4d9104"),
        );
        assert_eq!(pr.get_commit_range(), expected);
    }
}
