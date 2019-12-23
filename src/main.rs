mod docker;
mod pull_request;
mod repo;
use pull_request::PullRequest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    github_workspace: String,
    github_event_path: String,
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            1
        }
    });
}

fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let config = match envy::from_env::<Config>() {
        Ok(c) => c,
        Err(e) => return Err(Box::new(e)),
    };

    let pr = match PullRequest::read(config.github_event_path) {
        Err(e) => return Err(Box::new(e)),
        Ok(x) => x,
    };

    println!("base sha {}", pr.base.sha);
    let git = match repo::open(config.github_workspace) {
        Err(e) => return Err(Box::new(e)),
        Ok(x) => x,
    };

    let changed_files = match git.get_changed_files(pr) {
        Err(e) => return Err(Box::new(e)),
        Ok(f) => f,
    };

    println!("files changed {:?}", changed_files);
    let paths = docker::get_dockerfile_paths(changed_files);

    for path in paths {
        match path.to_str() {
            Some(s) => println!("{}", s),
            None => (),
        }
    }

    Ok(())
}
