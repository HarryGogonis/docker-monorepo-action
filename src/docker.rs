use std::fmt;
use std::process::Command;

#[derive(Debug)]
pub struct DockerError {
    stdout: String,
}

type Result = std::result::Result<(), DockerError>;

pub fn build(tag: String, path: String) -> Result {
    let mut docker = Command::new("docker");
    let cmd = docker.arg("build").arg("-t").arg(tag).arg(path);

    run_cmd(cmd)?;
    Ok(())
}

pub fn push(tag: String) -> Result {
    let mut docker = Command::new("docker");
    let cmd = docker.arg("push").arg(tag);

    run_cmd(cmd)?;
    Ok(())
}

fn run_cmd(cmd: &mut Command) -> Result {
    let output = cmd.output().expect("failed to execute docker process");

    match String::from_utf8(output.stdout) {
        Ok(o) => println!("{}", o),
        Err(_) => (),
    };

    if output.status.success() {
        Ok(())
    } else {
        let stdout = String::from_utf8(output.stderr).expect("could not read output from docker");
        Err(DockerError { stdout })
    }
}

impl fmt::Display for DockerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.stdout.fmt(f)
    }
}

impl std::error::Error for DockerError {}
