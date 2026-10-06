# `serde_json` 1.0.151 AST Receipts & Verified Patterns

> **Primary Source:** [`serde-rs/json` v1.0.151](https://github.com/serde-rs/json/tree/v1.0.151)  
> **Repository:** [serde-rs/json](https://github.com/serde-rs/json)  
> **Retrieved:** 2026-10-06 live via crates.io & GitHub  

---

## 1. Preserving Key Order (`preserve_order`)

Unreal Engine 5.1 JSON files produced by `UAssetGUI tojson` require stable serialization without alphabetical key reordering. Enabling the `preserve_order` feature uses an `IndexMap` internally instead of a `BTreeMap`:

```toml
[dependencies]
serde = { version = "1.0.229", features = ["derive"] }
serde_json = { version = "1.0.151", features = ["preserve_order"] }
```

---

## 2. Dynamic AST Manipulation via `serde_json::Value`

### Finding & Modifying Nested Values with `pointer_mut`
```rust
use serde_json::Value;

// JSON pointer RFC 6901 syntax: /Exports/0/Data/1/Value
if let Some(records) = json_root.pointer_mut("/Exports/0/Data/1/Value") {
    if let Some(arr) = records.as_array_mut() {
        // Direct manipulation of native JSON arrays
        arr.push(new_element);
    }
}
```

### Safe Array Splicing Without PowerShell Array Flattening
In PowerShell, `@($arr + (, $item))` caused array corruption (`{ "value": [...], "Count": 2 }`).
In `serde_json`:
```rust
let array = val.as_array_mut().expect("must be array");
let clone = array[0].clone();
array.push(clone); // Appends as pure JSON array element: [elem1, elem2, clone]
```

---

## 3. Pretty-Printing JSON to Disk

```rust
use std::fs::File;
use std::io::BufWriter;
use serde_json::Value;

pub fn write_pretty_json(path: &std::path::Path, value: &Value) -> anyhow::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, value)?;
    Ok(())
}
```
