---
name: serde-json-ast
description: Mandatory before authoring or modifying Unreal Engine JSON asset transformations. Governs serde_json 1.0.151 AST manipulation, array splicing, and preservation of object key order.
---

# Skill: Serde JSON AST Manipulator (`serde-json-ast`)

> **Status:** AUDITED & ACTIVE  
> **Crate Baseline:** `serde_json 1.0.151` (features: `["preserve_order"]`), `serde 1.0.229` (features: `["derive"]`)  
> **Reference Folder:** [`docs/dependencies/serde-json/`](../../../docs/dependencies/serde-json/README.md)  
> **Primary Source:** [`serde-rs/json` v1.0.151](https://github.com/serde-rs/json/tree/v1.0.151)  

Every rule below cites its empirical receipt. If an API or pattern is not cited here or in `docs/dependencies/serde-json/`, it is not verified. Do not invent it (see `AGENTS.md` §1).

---

## 1. Scope Boundaries

**Use this skill for:**
* Reading, mutating, and writing Unreal Engine 5.1 JSON asset files serialized by `UAssetGUI`.
* Traversing complex JSON structures via JSON Pointer RFC 6901 (`.pointer()` and `.pointer_mut()`).
* Splicing array records for `DragonAdventureIFChartData` and character registry entries.
* Pretty-printing formatted JSON output back to disk for consumption by `UAssetGUI fromjson`.

**Do NOT use this skill for:**
* CLI argument parsing (delegate to `clap-cli-parser`).
* Error wrapping and context attachment (delegate to `anyhow-error-handling`).

---

## 2. Verified Golden Paths (serde_json 1.0.151)

### 2.1 Crate Dependency Configuration
In `crates/complete-story-cli/Cargo.toml`:
```toml
[dependencies]
serde = { version = "1.0.229", features = ["derive"] }
serde_json = { version = "1.0.151", features = ["preserve_order"] }
```

### 2.2 In-Memory AST Mutation Pattern
```rust
use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub fn modify_chart_ast(path: &Path, route_key: &str) -> Result<()> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open JSON at {:?}", path))?;
    let mut root: Value = serde_json::from_reader(BufReader::new(file))
        .with_context(|| format!("Failed to parse JSON AST from {:?}", path))?;

    // RFC 6901 Pointer lookup
    let records = root
        .pointer_mut("/Exports/0/Data/1/Value")
        .and_then(Value::as_array_mut)
        .context("Missing /Exports/0/Data/1/Value array in ChartData AST")?;

    // Clone Goku baseline entry
    let mut new_entry = records[0].clone();
    new_entry[0]["Value"][0]["Value"] = json!(route_key);

    // Native array push - zero wrapping overhead
    records.push(new_entry);

    let out_file = File::create(path)?;
    serde_json::to_writer_pretty(BufWriter::new(out_file), &root)?;
    Ok(())
}
```

---

## 3. Negative Constraints (Hallucination Defense)

| Forbidden | Why (Receipt) | Do Instead |
|---|---|---|
| Omitting `preserve_order` feature | Default BTreeMap re-sorts JSON keys alphabetically, confusing UAssetGUI | Always enable `features = ["preserve_order"]` |
| Wrapping nested array records in custom objects | Recreates PowerShell's `{ "value": [...], "Count": 2 }` bug | Mutate `Value::Array` directly via `as_array_mut()` |
| Discarding JSON formatting (`to_string()` without newlines) | Produces unreadable minified diffs in staging | Use `serde_json::to_writer_pretty` |
| Monolithic transformation files exceeding 300 lines | Violates repo hygiene rule (`AGENTS.md` §0.4) | Decompose transformations into separate domain modules |
