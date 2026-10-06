---
name: anyhow-error-handling
description: Mandatory before implementing fallible functions or subprocess handlers in crates/complete-story-cli. Governs anyhow 1.0.104 error context chaining and diagnostic reporting.
---

# Skill: Anyhow Error Handling (`anyhow-error-handling`)

> **Status:** AUDITED & ACTIVE  
> **Crate Baseline:** `anyhow 1.0.104`  
> **Reference Folder:** [`docs/dependencies/anyhow/`](../../../docs/dependencies/anyhow/README.md)  
> **Primary Source:** [`dtolnay/anyhow` v1.0.104](https://github.com/dtolnay/anyhow/tree/1.0.104)  

Every rule below cites its empirical receipt. If a method or macro pattern is not cited here or in `docs/dependencies/anyhow/`, it is not verified. Do not invent it (see `AGENTS.md` §1).

---

## 1. Scope Boundaries

**Use this skill for:**
* Returning `anyhow::Result<T>` from fallible functions across `crates/complete-story-cli`.
* Attaching high-level operational context using `.with_context(|| ...)` and `.context(...)`.
* Raising explicit operational failures using `bail!("...")` or `anyhow!("...")`.
* Formatting subprocess exit codes, stdout, and stderr for user diagnostics.

**Do NOT use this skill for:**
* Swallowing errors silently with `let _ = ...`.
* Panicking with `.unwrap()` or `.expect()` in non-test code.

---

## 2. Verified Golden Paths (anyhow 1.0.104)

### 2.1 Crate Dependency Configuration
In `crates/complete-story-cli/Cargo.toml`:
```toml
[dependencies]
anyhow = "1.0.104"
```

### 2.2 Error Context Pattern
From `docs/dependencies/anyhow/context-receipts.md`:
```rust
use anyhow::{Context, Result};
use std::path::Path;

pub fn read_asset(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read asset payload from path: {:?}", path))
}
```

### 2.3 Checked Process Runner
```rust
use anyhow::{bail, Context, Result};
use std::process::Command;

pub fn run_checked(cmd: &mut Command, label: &str) -> Result<String> {
    let output = cmd
        .output()
        .with_context(|| format!("Failed to spawn process for {}", label))?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "Subprocess '{}' failed with exit code {:?}.\nSTDOUT: {}\nSTDERR: {}",
            label,
            output.status.code(),
            stdout.trim(),
            stderr.trim()
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
```

---

## 3. Negative Constraints (Hallucination Defense)

| Forbidden | Why (Receipt) | Do Instead |
|---|---|---|
| Using `.unwrap()` on subprocess or I/O results | Causes immediate, unhelpful panics without error context | Use `?` operator returning `anyhow::Result` |
| Falling back to fake/mock/synthetic data on failure | Masks real defects, creates invisible bugs, and blocks diagnostics | Fail hard immediately (`bail!`) with path, command, and stderr |
| Calling `std::process::exit(1)` directly in helper functions | Skips RAII drop destructors and truncates logs | Return `Result<()>` and let `main()` exit cleanly |
| Silently ignoring child process failures | Recreates the PowerShell silent failure bug | Assert `output.status.success()` or `bail!` |
