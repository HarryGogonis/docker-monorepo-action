mod docker;
mod event;
mod repo;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    workspace: String,
    event_path: String,
    repository: String,
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
    let config = match envy::prefixed("GITHUB_").from_env::<Config>() {
        Ok(c) => c,
        Err(e) => return Err(Box::new(e)),
    };

    let event = match event::read(config.event_path.clone(), config.workspace.clone()) {
        Err(e) => return Err(Box::new(e)),
        Ok(x) => x,
    };

    let git = match repo::open(config.workspace.clone()) {
        Err(e) => return Err(Box::new(e)),
        Ok(x) => x,
    };

    let paths = match git.get_dockerfile_paths(event) {
        Err(e) => return Err(Box::new(e)),
        Ok(x) => x,
    };

    for path in paths {
        if let (Some(app), Some(p)) = (path.file_name().and_then(|p| p.to_str()), path.to_str()) {
            let tag = format!(
                "docker.pkg.github.com/{}/{}:{}",
                config.repository.to_lowercase(),
                app,
                "master"
            );

            docker::build(tag.clone(), String::from(p));
            docker::push(tag.clone());
        }
    }

    Ok(())
}
