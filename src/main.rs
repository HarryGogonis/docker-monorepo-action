mod docker;
mod event;
mod repo;
use crate::repo::Repo;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    workspace: String,
    event_path: String,
    event_name: String,
    repository: String,
    actor: String,
    token: String,
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
    // TODO consider passing these in
    let docker_registry = "docker.pkg.github.com";
    let build_tag = "master";

    let config = match envy::prefixed("GITHUB_").from_env::<Config>() {
        Ok(c) => c,
        Err(e) => return Err(Box::new(e)),
    };

    let event = match event::read(
        config.event_path.clone(),
        config.event_name.clone(),
        config.workspace.clone(),
    ) {
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

    if paths.len() <= 0 {
        return Ok(());
    }

    docker::login(config.actor, config.token, String::from(docker_registry))?;

    for path in paths {
        if let (Some(app), Some(p)) = (path.file_name().and_then(|p| p.to_str()), path.to_str()) {
            let tag = format!(
                "{}/{}/{}:{}",
                docker_registry,
                config.repository.to_lowercase(),
                app,
                build_tag
            );

            docker::build(tag.clone(), String::from(p))?;
            docker::push(tag.clone())?;
        }
    }

    Ok(())
}
