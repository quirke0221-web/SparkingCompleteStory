# `anyhow` 1.0.104 Context Receipts & Verified Patterns

> **Primary Source:** [`dtolnay/anyhow` v1.0.104](https://github.com/dtolnay/anyhow/tree/1.0.104)  
> **Repository:** [dtolnay/anyhow](https://github.com/dtolnay/anyhow)  
> **Retrieved:** 2026-10-06 live via crates.io & GitHub  

---

## 1. Core Receipts (`README.md`)

```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let content = std::fs::read(path)
        .with_context(|| format!("Failed to read instrs from {}", path))?;
    Ok(())
}
```

```rust
use anyhow::bail;

if !status.success() {
    bail!("Command failed with exit code: {}", status);
}
```

---

## 2. Subprocess Error Enrichment Pattern

In `crates/complete-story-cli`, third-party tools like `retoc` and `UAssetGUI` must have their outputs captured and presented with clear diagnostics on failure:

```rust
use anyhow::{bail, Context, Result};
use std::process::Command;

pub fn execute_checked(cmd: &mut Command, task_name: &str) -> Result<String> {
    let output = cmd
        .output()
        .with_context(|| format!("Failed to spawn process for task: {}", task_name))?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "Task '{}' exited with code {}.\n--- STDOUT ---\n{}\n--- STDERR ---\n{}",
            task_name,
            output.status.code().unwrap_or(-1),
            stdout.trim(),
            stderr.trim()
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
```
