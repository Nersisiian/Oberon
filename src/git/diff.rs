use crate::Result;
use std::process::Command;

pub fn get_diff(staged: bool) -> Result<String> {
    let mut args = vec!["diff"];
    if staged {
        args.push("--staged");
    }
    args.push("--no-color");

    let output = Command::new("git").args(&args).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
