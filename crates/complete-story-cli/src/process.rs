use anyhow::{bail, Context, Result};
use std::process::{Command, Output};

pub fn run_checked(cmd: &mut Command, label: &str) -> Result<Output> {
    let output = cmd
        .output()
        .with_context(|| format!("Failed to spawn process for task: {}", label))?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "Task '{}' failed with exit code {:?}.\n--- STDOUT ---\n{}\n--- STDERR ---\n{}",
            label,
            output.status.code(),
            stdout.trim(),
            stderr.trim()
        );
    }

    Ok(output)
}

pub fn run_checked_stdout(cmd: &mut Command, label: &str) -> Result<String> {
    let output = run_checked(cmd, label)?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
