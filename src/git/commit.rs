use crate::Result;
use std::process::Command;

pub fn commit(message: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .output()?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(crate::OberonError::Git(git2::Error::from_str(&err)));
    }
    Ok(())
}
