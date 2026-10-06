# Rust Toolchain & Core Modding Crates Reference

> **Toolchain Baseline:** `rustc 1.98.1` / `cargo 1.98.1` (x86_64-pc-windows-msvc)  
> **Crate Target:** `crates/complete-story-cli`  
> **Status:** AUDITED & PINNED  

---

## 1. Toolchain Role & Scope

The Rust toolchain replaces the legacy PowerShell build harness (`helpers/*.ps1`) with an authoritative, strongly-typed native CLI binary (`complete-story-cli`). It is responsible for:
* Subprocess orchestration (`retoc`, `UAssetGUI`) with strict exit-code and stderr verification.
* Type-safe, deterministic Unreal Engine 5.1 JSON asset transformations (`serde_json`).
* Container packing, verification, and staging to Steam directories.
* Packaging distributable release archives (`zip`).

---

## 2. Core Crate Ingestion & Pinned Versions

All crates are pinned to verified, stable releases on [crates.io](https://crates.io):

| Crate | Pinned Version | Features | Primary Source / Repository | Rationale & Functionality |
| :--- | :--- | :--- | :--- | :--- |
| **`clap`** | `4.5` | `["derive"]` | [`clap-rs/clap`](https://github.com/clap-rs/clap) | Declarative CLI argument parsing. Provides type-safe subcommands (`build`, `deploy`, `logs`, `pack`). |
| **`serde`** | `1.0` | `["derive"]` | [`serde-rs/serde`](https://github.com/serde-rs/serde) | Generic serialization framework powering JSON data manipulation. |
| **`serde_json`** | `1.0` | `["preserve_order"]` | [`serde-rs/json`](https://github.com/serde-rs/json) | AST parser and serializer (`serde_json::Value`). Guarantees no array-flattening or administrative wrapper injection. |
| **`anyhow`** | `1.0` | default | [`dtolnay/anyhow`](https://github.com/dtolnay/anyhow) | Idiomatic, expressive error reporting with rich `.context(...)` chains and automatic backtraces. |
| **`zip`** | `2.2` | `["deflate"]` | [`zip-rs/zip2`](https://github.com/zip-rs/zip2) | High-performance creation and reading of ZIP release archives. |
| **`walkdir`** | `2.5` | default | [`BurntSushi/walkdir`](https://github.com/BurntSushi/walkdir) | Efficient, cross-platform recursive directory traversal. |
| **`colored`** | `2.1` | default | [`colored-rs/colored`](https://github.com/colored-rs/colored) | ANSI color formatting for terminal logs and CLI status reporting. |

---

## 3. Verified Code Receipts & Golden Patterns

### 3.1 Subprocess Execution Pattern (`process.rs`)
Subprocesses must never fail silently. Every process execution must capture stdout, stderr, and assert exit code zero:

```rust
use anyhow::{bail, Context, Result};
use std::process::Command;

pub fn run_checked(cmd: &mut Command, description: &str) -> Result<String> {
    let output = cmd
        .output()
        .with_context(|| format!("Failed to launch process for: {}", description))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        bail!(
            "Command failed [exit {}] for {}:\nSTDOUT: {}\nSTDERR: {}",
            output.status.code().unwrap_or(-1),
            description,
            stdout,
            stderr
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
```

### 3.2 Pure AST Transformation Pattern (`transform/`)
Using `serde_json::Value` prevents PowerShell's array-mangling behavior:

```rust
use anyhow::{Context, Result};
use serde_json::{json, Value};

pub fn splice_chart_entry(json: &mut Value, route_key: &str) -> Result<()> {
    let records = json
        .pointer_mut("/Exports/0/Data/1/Value")
        .and_then(Value::as_array_mut)
        .context("Could not find PtrRecords array in ChartData JSON")?;

    // Clone Goku's baseline record
    let mut new_record = records[0].clone();
    
    // Modify Key property safely without object wrapping
    new_record[0]["Value"][0]["Value"] = json!(route_key);
    
    // Push as native array element [StructData, ObjectData]
    records.push(new_record);
    Ok(())
}
```

---

## 4. Hallucination Prevention & Negative Constraints

1. **No Raw `.unwrap()` in Production:** Every fallible operation (I/O, process spawn, JSON access) must return a `Result` and attach context with `anyhow::Context`.
2. **No Hardcoded AES Key:** The AES decryption key must be read from the environment variable (`SPARKING_ZERO_AES_KEY`), never hardcoded in Rust source code or committed to Git.
3. **No File Exceeding 300 Lines:** Every source file under `crates/complete-story-cli/src/` must remain `<= 300` lines per `AGENTS.md` Rule 0.4.
4. **Preserve Exact Casing on Paths:** Unreal Engine asset paths (e.g. `/Game/SS/MasterDataAsset/...`) are case-sensitive inside Zen container packages. Never normalize or change case.
