mod pull_request;
use std::io::{Error, ErrorKind, Result};

fn main() -> Result<()> {
    let pr = match pull_request::read_file(
        "/Users/harrygogonis/Code/Rust/docker-monorepo-action/test/payload.json",
    ) {
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        Ok(data) => data,
    };

    println!("head sha {}", pr.head.sha);
    Ok(())
}
