# `zip` 8.6.0 Archive Receipts & Verified Patterns

> **Primary Source:** [`zip-rs/zip2` v8.6.0](https://github.com/zip-rs/zip2/tree/v8.6.0)  
> **Repository:** [zip-rs/zip2](https://github.com/zip-rs/zip2)  
> **Receipt File:** `examples/write_sample.rs`  
> **Retrieved:** 2026-10-06 live via crates.io & GitHub  

---

## 1. Verbatim Archive Creation Pattern (`write_sample.rs`)

```rust
use std::fs::File;
use std::io::Write;
use std::path::Path;
use zip::CompressionMethod::Deflated;
use zip::write::{SimpleFileOptions, ZipWriter};

pub fn create_zip(output_path: &Path) -> anyhow::Result<()> {
    let file = File::create(output_path)?;
    let mut zip = ZipWriter::new(file);

    let options = SimpleFileOptions::default()
        .compression_method(Deflated)
        .unix_permissions(0o755);

    zip.start_file("test.txt", options)?;
    zip.write_all(b"Hello world\n")?;

    zip.finish()?;
    Ok(())
}
```

---

## 2. Directory Packaging Pattern

Using standard library `std::fs::read_dir` to package a directory recursively without extra crates:

```rust
use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::CompressionMethod::Deflated;
use zip::write::{SimpleFileOptions, ZipWriter};

pub fn zip_directory(src_dir: &Path, zip_path: &Path) -> Result<()> {
    let file = File::create(zip_path)
        .with_context(|| format!("Failed to create zip file at {:?}", zip_path))?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(Deflated);

    fn add_dir(
        zip: &mut ZipWriter<File>,
        base: &Path,
        current: &Path,
        options: SimpleFileOptions,
    ) -> Result<()> {
        for entry in std::fs::read_dir(current)? {
            let entry = entry?;
            let path = entry.path();
            let relative = path.strip_prefix(base)?.to_string_lossy().replace('\\', "/");

            if path.is_dir() {
                zip.add_directory(format!("{}/", relative), options)?;
                add_dir(zip, base, &path, options)?;
            } else {
                zip.start_file(relative, options)?;
                let bytes = std::fs::read(&path)?;
                zip.write_all(&bytes)?;
            }
        }
        Ok(())
    }

    add_dir(&mut zip, src_dir, src_dir, options)?;
    zip.finish()?;
    Ok(())
}
```
