---
name: rust-pipeline-builder
description: Mandatory before ANY Rust CLI modifications or invocations in crates/complete-story-cli. Governs native build pipeline orchestration, strongly-typed JSON asset transformations, and Zen IoStore container packaging.
---

# Skill: Rust Pipeline Builder (`complete-story-cli`)

> **Status:** AUDITED & ACTIVE  
> **Toolchain Baseline:** `rustc 1.98.1` / `cargo 1.98.1` (x86_64-pc-windows-msvc)  
> **Crate Target:** `crates/complete-story-cli`  
> **Reference Folder:** [`docs/dependencies/rust-toolchain/`](../../../docs/dependencies/rust-toolchain/README.md)  
> **Master Strategy:** [`REFACTOR_STRATEGY_RUST_TOOLCHAIN.md`](../../../REFACTOR_STRATEGY_RUST_TOOLCHAIN.md)  

Every rule below cites its empirical receipt. If an API or CLI command is not cited here or in `docs/dependencies/rust-toolchain/`, it is not verified. Do not invent it (see `AGENTS.md` §1).

---

## 1. Scope Boundaries

**Use this skill for:**
* Compiling and executing the native modding CLI (`cargo run -p complete-story-cli -- <command>`).
* Authoring or editing Rust source files under `crates/complete-story-cli/src/`.
* Transforming Unreal Engine 5.1 asset JSON files via `serde_json::Value`.
* Orchestrating third-party subprocesses (`retoc`, `UAssetGUI`) with exit code and stream verification.
* Packaging and deploying mod release archives (`.zip`) and Zen container files (`.pak`, `.utoc`, `.ucas`).

**Do NOT use this skill for (hand off instead):**
| Task | Owner skill |
|---|---|
| CLI argument parsing & subcommands | `clap-cli-parser` |
| JSON AST parsing & mutation | `serde-json-ast` |
| Error propagation & subprocess diagnostics | `anyhow-error-handling` |
| Distributable zip archive packaging | `zip-archive-packager` |
| In-game Lua scripting and GameThread reflection | `ue4ss-runtime-scripting` |
| Low-level IoStore container byte format rules | `retoc-iostore-packer` |
| Low-level `.usmap` schema mapping definitions | `uassetgui-asset-serialization` |
| End-to-end mod engineering lifecycle coordination | `sparking-zero-mod-pipeline` |

---

## 2. Verified Commands (Golden Paths)

### 2.1 Full Build and Deploy
```bash
cargo run -p complete-story-cli -- build --deploy
```
* **Receipt:** Extracts stock assets from game archives, converts `.uasset` to JSON, runs AST transformations, compiles back to `.uasset`, packs Zen containers via `retoc`, and deploys to `~mods/` and `Binaries/Win64/`.

### 2.2 Iterative Fast Build (Skip Extraction)
```bash
cargo run -p complete-story-cli -- build --skip-extract --deploy
```
* **Receipt:** Bypasses Stage 1 extraction when `build/staging/legacy/` assets are already staged, saving ~15 seconds on rebuilds.

### 2.3 Live Mod Log Streaming
```bash
cargo run -p complete-story-cli -- logs
```
* **Receipt:** Streams live entries from `SparkingZERO\Binaries\Win64\ue4ss.log` directly to terminal output.

---

## 3. Engineering Invariants & Coding Standards

1. **Strict 300-Line Code Ceiling (`AGENTS.md` §0.4):** Every `.rs` file in `crates/complete-story-cli/src/` must remain `<= 300` lines. Decompose monolithic logic into focused domain modules (`config.rs`, `process.rs`, `transform/`, `container.rs`, `deploy.rs`, `release.rs`).
2. **Zero Raw `.unwrap()` in Production:** Every fallible operation (I/O, process execution, JSON access) must return `anyhow::Result<T>` and attach context using `.with_context(...)`.
3. **No Array-Flattening or Administrative Wrapper Bugs:** AST mutations in `transform/` must operate on `serde_json::Value::Array`. Never wrap nested array pairs in custom objects.
4. **Environment-Sourced AES Key:** Read `SPARKING_ZERO_AES_KEY` from `std::env::var`. Never hardcode the AES key in source files or commit it to Git.
5. **Exact Path Casing:** Unreal Engine asset paths (e.g. `/Game/SS/MasterDataAsset/...`) are case-sensitive. Preserve exact original casing on all package paths.

---

## 4. Negative Constraints (Hallucination Defense)

| Forbidden | Why (Receipt) | Do Instead |
|---|---|---|
| Using `unwrap()` or `expect()` on process outputs | Panics on missing binaries without diagnostic logs | Return `anyhow::Result` with `.context()` |
| Hardcoding `0xb240...` AES key in Rust source | Violates project security invariant (`AGENTS.md` §5) | Read from `std::env::var("SPARKING_ZERO_AES_KEY")` |
| Writing `.rs` files exceeding 300 lines | Violates repo hygiene rule (`AGENTS.md` §0.4) | Decompose into modular domain files |
| Silently swallowing subprocess errors | Caused hours of debugging in PowerShell | Check `status.success()` and dump stdout/stderr |
| Modifying vanilla save files (`MainGameSaveData`) | Violates non-destructive coexistence (`AGENTS.md` §4.3) | Mount via `~mods/CompleteStory/` and UE4SS hooks |
