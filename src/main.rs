mod pull_request;
mod repo;
use std::io::{Error, ErrorKind, Result};

fn main() -> Result<()> {
    let pr = match pull_request::new(
        // todo do not hardcode
        "/Users/harrygogonis/Projects/Rust/docker-monorepo-action/test/payload.json",
    ) {
        Err(e) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Failed to load PR JSON: {}", e),
            ))
        }
        Ok(x) => x,
    };

    println!("base sha {}", pr.base.sha);
    let git = match repo::new("/Users/harrygogonis/Projects/htpc-docker") {
        Err(e) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Failed to load git repo: {}", e),
            ))
        }
        Ok(x) => x,
    };

    // todo do not hardcode
    match git.get_changed_files(pr) {
        Err(e) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Failed to diff: {}", e),
            ))
        }
        Ok(changed_files) => println!("files changed {:?}", changed_files),
    }

    Ok(())
}
