use crate::event::{EventError, GitRef};
use crate::repo::Diffable;
use serde::{Deserialize, Serialize};

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

impl PullRequest {
    pub fn read(json: String) -> Result<PullRequest, EventError> {
        let v: PullRequest = match serde_json::from_str(&json) {
            Ok(pr) => pr,
            Err(e) => return Err(EventError::JSON(e)),
        };
        Ok(v)
    }
}
