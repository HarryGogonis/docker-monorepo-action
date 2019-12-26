use crate::event::{EventError, EventParser, EventPayload};
use crate::repo::Diffable;
use serde::{Deserialize, Serialize};

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

impl Diffable for PullRequest {
    fn get_commit_range(&self) -> (String, String) {
        let base_commit = self.base.sha.clone();
        let head_commit = self.head.sha.clone();
        (base_commit, head_commit)
    }
}

impl EventParser for PullRequest {
    fn read(json: &str) -> Result<EventPayload, EventError> {
        let v: PullRequest = match serde_json::from_str(json) {
            Ok(pr) => pr,
            Err(e) => return Err(EventError::JSON(e)),
        };
        Ok(Box::new(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_util;

    #[test]
    fn test_read() {
        let json = file_util::read("./fixtures/pull_request.json").unwrap();

        let pr = PullRequest::read(&json).unwrap();

        let expected = (
            String::from("c467a407711867b6b9a2df8bf54c548587ff54ed"),
            String::from("a08ade107ca1d97f80c42c2e44abc7a38d4d9104"),
        );
        assert_eq!(pr.get_commit_range(), expected);
    }
}
