---
name: zip-archive-packager
description: Mandatory before creating or modifying mod distribution archive packaging in crates/complete-story-cli. Governs zip 8.6.0 archive creation and SimpleFileOptions.
---

# Skill: Zip Archive Packager (`zip-archive-packager`)

> **Status:** AUDITED & ACTIVE  
> **Crate Baseline:** `zip 8.6.0` (`default-features = false`, features: `["deflate"]`)  
> **Reference Folder:** [`docs/dependencies/zip/`](../../../docs/dependencies/zip/README.md)  
> **Primary Source:** [`zip-rs/zip2` v8.6.0](https://github.com/zip-rs/zip2/tree/v8.6.0)  

Every rule below cites its empirical receipt. If a method or struct is not cited here or in `docs/dependencies/zip/`, it is not verified. Do not invent it (see `AGENTS.md` §1).

---

## 1. Scope Boundaries

**Use this skill for:**
* Compressing mod assets into standalone `.zip` distribution bundles in `build/dist/`.
* Setting entry compression methods (`Deflated`).
* Constructing forward-slash archive paths compatible with Windows and Linux zip extractors.

**Do NOT use this skill for:**
* Unreal Engine Zen container packaging (`.pak`, `.utoc`, `.ucas` must use `retoc-iostore-packer`).
* CLI parsing or error handling (delegate to `clap-cli-parser` or `anyhow-error-handling`).

---

## 2. Verified Golden Paths (zip 8.6.0)

### 2.1 Crate Dependency Configuration
In `crates/complete-story-cli/Cargo.toml`:
```toml
[dependencies]
zip = { version = "8.6.0", default-features = false, features = ["deflate"] }
```

### 2.2 Directory Compression Pattern
From `docs/dependencies/zip/archive-receipts.md`:
```rust
use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use zip::CompressionMethod::Deflated;
use zip::write::{SimpleFileOptions, ZipWriter};

pub fn build_release_zip(source_dir: &Path, output_zip: &Path) -> Result<()> {
    let file = File::create(output_zip)
        .with_context(|| format!("Failed to create zip file at {:?}", output_zip))?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(Deflated);

    // Recursively add directory entries using forward-slash normalization
    for entry in std::fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy();
            zip.start_file(file_name, options)?;
            let bytes = std::fs::read(&path)?;
            zip.write_all(&bytes)?;
        }
    }

    zip.finish().context("Failed to finalize zip archive")?;
    Ok(())
}
```

---

## 3. Negative Constraints (Hallucination Defense)

| Forbidden | Why (Receipt) | Do Instead |
|---|---|---|
| Using `FileOptions` from older `zip` versions | Deprecated in `zip` 8.x in favor of `SimpleFileOptions` | Use `zip::write::SimpleFileOptions::default()` |
| Using `zip 9.0.0-pre3` | Unstable pre-release with breaking changes | Pin stable `zip 8.6.0` |
| Using backslashes (`\`) in zip archive paths | Breaks cross-platform extraction on Linux/Unverum | Convert backslashes to forward slashes (`/`) |
| Writing archive logic exceeding 300 lines | Violates repo hygiene rule (`AGENTS.md` §0.4) | Keep archive packaging isolated in `src/release.rs` |
