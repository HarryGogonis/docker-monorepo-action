mod pull_request;
mod repo;
use pull_request::{PullRequest};

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            1
        }
    });
}

fn run_app() -> Result<(),  Box<dyn std::error::Error>> {
    let pr = match PullRequest::read(
        // todo do not hardcode
        "./test/payload.json",
    ) {
        Err(e) => return Err(Box::new(e)),
        Ok(x) => x,
    };

    println!("base sha {}", pr.base.sha);
    let git = match repo::new("/Users/harrygogonis/Projects/htpc-docker") {
        Err(e) => return Err(Box::new(e)),
        Ok(x) => x,
    };

    // todo do not hardcode
    match git.get_changed_files(pr) {
        Err(e) => return Err(Box::new(e)),
        Ok(changed_files) => println!("files changed {:?}", changed_files),
    }

    Ok(())
}
