use crate::pull_request::PullRequest;
use git2::{DiffDelta, Error, Repository, Tree};
use std::vec::Vec;

pub trait Repo {
    fn get_tree(&self, sha: &str) -> Result<Tree, Error>;
    fn get_changed_files(&self, pr: PullRequest) -> Result<Vec<String>, Error>;
}

pub struct GitRepo {
    repo: Repository,
}

impl Repo for GitRepo {
    fn get_changed_files(&self, pr: PullRequest) -> Result<Vec<String>, Error> {
        let mut v = Vec::new();
        let base = self.get_tree(&pr.base.sha)?;
        let head = self.get_tree(&pr.head.sha)?;

        // TODO do something with this value
        let diff = self
            .repo
            .diff_tree_to_tree(Some(&base), Some(&head), None)?;

        let mut push_file = |delta: DiffDelta, _: f32| -> bool {
            let path = match delta.new_file().path() {
                Some(p) => p,
                None => return false,
            };

            match path.to_str() {
                Some(p) => v.push(String::from(p)),
                None => return false,
            };

            return true;
        };

        let _ = diff.foreach(&mut push_file, None, None, None)?;

        Ok(v)
    }

    fn get_tree(&self, sha: &str) -> Result<Tree, Error> {
        let obj = self.repo.revparse_single(sha)?;
        let tree = obj.peel_to_tree()?;
        Ok(tree)
    }
}

pub fn new(file_path: &str) -> Result<Box<dyn Repo>, Error> {
    let repo = Repository::open(file_path)?;
    let git = Box::new(GitRepo { repo });
    Ok(git)
}
