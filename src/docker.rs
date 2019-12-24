use std::process::Command;

#[derive(Debug)]
pub enum DockerError {
    Build(String),
}

type Result = std::result::Result<(), DockerError>;

pub fn build(tag: String, path: String) -> Result {
    let cmd = format!("docker build -t {} {}", tag, path);
    println!("{}", cmd);
    let output = Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(tag)
        .arg(path)
        .output()
        .expect("failed to execute docker process");

    match String::from_utf8(output.stdout) {
        Ok(o) => println!("{}", o),
        Err(_) => (),
    };

    if output.status.success() {
        Ok(())
    } else {
        let stdout = String::from_utf8(output.stderr).expect("failed to read stdout");
        Err(DockerError::Build(stdout))
    }
}

pub fn push(tag: String) -> Result {
    let cmd = format!("docker push -{}", tag);
    println!("{}", cmd);
    let output = Command::new("docker")
        .arg("push")
        .arg(tag)
        .output()
        .expect("failed to execute docker process");

    match String::from_utf8(output.stdout) {
        Ok(o) => println!("{}", o),
        Err(_) => (),
    };

    if output.status.success() {
        Ok(())
    } else {
        let stdout = String::from_utf8(output.stderr).expect("failed to read stdout");
        Err(DockerError::Build(stdout))
    }
}
