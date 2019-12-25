use crate::event::Event;
use git2::{DiffDelta, Error, Repository, Tree};
use std::path::PathBuf;
use std::vec::Vec;

pub trait Diffable {
    fn get_commit_range(&self) -> (String, String);
}

pub trait Repo {
    fn get_tree(&self, sha: &str) -> Result<Tree, Error>;
    fn get_changed_files(&self, d: Box<dyn Diffable>) -> Result<Vec<PathBuf>, Error>;
    fn get_dockerfile_paths(&self, event: Event) -> Result<Vec<PathBuf>, Error>;
}

pub struct GitRepo {
    repo: Repository,
}

impl Repo for GitRepo {
    fn get_changed_files(&self, d: Box<dyn Diffable>) -> Result<Vec<PathBuf>, Error> {
        let mut v = Vec::new();
        let (base_commit, head_commit) = d.get_commit_range();
        let base = self.get_tree(&base_commit)?;
        let head = self.get_tree(&head_commit)?;

        let diff = self
            .repo
            .diff_tree_to_tree(Some(&base), Some(&head), None)?;

        let mut push_file = |delta: DiffDelta, _: f32| -> bool {
            match delta.new_file().path() {
                Some(p) => v.push(p.to_path_buf()),
                None => return true,
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

    fn get_dockerfile_paths(&self, event: Event) -> Result<Vec<PathBuf>, Error> {
        let files = self.get_changed_files(event.payload)?;

        let base_path = PathBuf::from(event.path);

        let mut v = Vec::new();

        for file in files {
            let mut repo_path = file.clone();
            repo_path.pop();

            let mut dir = base_path.clone();
            dir.push(repo_path);

            let mut dockerfile = dir.clone();
            dockerfile.push("Dockerfile");

            if dockerfile.exists() {
                v.push(dir);
            }
        }

        v.sort();
        v.dedup();

        Ok(v)
    }
}

pub fn open(file_path: String) -> Result<Box<GitRepo>, Error> {
    let repo = Repository::open(file_path)?;
    let git = Box::new(GitRepo { repo });
    Ok(git)
}
