use std::path::PathBuf;

pub fn get_dockerfile_paths(files: Vec<PathBuf>) -> Vec<PathBuf> {
    // TODO read from environment
    let base_path = PathBuf::from("/Users/harrygogonis/Projects/htpc-docker/");
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

    return v;
}
