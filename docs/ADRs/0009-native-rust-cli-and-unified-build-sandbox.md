# ADR 0009: Native Rust CLI Toolchain and Unified Build Sandbox

## Status
Accepted (Supersedes [ADR 0003](0003-separation-of-concerns-in-asset-pipeline.md) and [ADR 0008](0008-accessforge-architectural-parity-helpers-and-root-mod.md))

## Context
Following the consolidation of legacy scripts in ADR 0003 and ADR 0008, the build and asset transformation pipeline relied on Windows PowerShell scripts (`helpers/Build-CompleteStory.ps1`, `Transform-CompleteStoryAssets.ps1`, `Deploy-DevelopmentBuild.ps1`, `Get-ModLogs.ps1`) and configuration manifests (`config/project.local.psd1`).

Several structural limitations and architectural frictions became apparent:
1. **PowerShell AST Serialization Brittleness:** PowerShell's object pipeline frequently converted single-element arrays to scalars or wrapped nested arrays into administrative hash wrappers (e.g. `{ value: [...], Count: 2 }`), breaking Unreal Engine's unversioned property structures during `fromjson` recompilation.
2. **Lack of Compile-Time Guarantees:** PowerShell scripts lacked static typing, schema enforcement, and compile-time verification. Regressions in JSON mutation logic were only caught at runtime after slow external CLI invocations.
3. **Root Directory Clutter & Sprawl:** The repository root accumulated scattered staging and output directories (`staging/`, `dist/`, `target/`, `tools/`, `helpers/`, `config/`), violating the Clean Root Invariant (`AGENTS.md` §0.3) and confusing human playtesters and autonomous agents.
4. **Third-Party Binary Visibility:** Portable modding binaries (`retoc.exe`, `UAssetGUI.exe`, and mapping files) resided in an untracked root `tools/` folder that was visually conflated with repository source code.

## Decision
1. **Authoritative Native Rust CLI (`crates/complete-story-cli`):**
   * Replaced the entire PowerShell toolchain with a unified, strongly-typed native Rust CLI located at `crates/complete-story-cli`.
   * CLI argument parsing is governed by `clap 4.6.7` derive macros (`Build`, `Pack`, `Deploy`, `Logs`).
   * Process execution and error context chaining are governed by `anyhow 1.0.104` with fail-hard diagnostics (zero mock data, zero swallowed exit codes).
   * Standalone release packaging is governed by `zip 8.6.0` (Deflate compression with forward-slash normalization).
2. **Strongly-Typed In-Memory AST Transformations (`src/transform/`):**
   * Asset transformations operate directly on `serde_json::Value::Array` in memory without disk roundtrips.
   * Enforces exact array sizing (13 chart elements, 14 registry entries), preserves Base64 `RawExport` bytes, and guarantees clean binary recompilation without array-wrapping bugs.
   * Enforced via automated unit and integration tests (`cargo test -p complete-story-cli`).
3. **Unified Build Sandbox (`build/`):**
   * All transient, generated build artifacts are consolidated under a single root `build/` directory excluded from Git via `.gitignore`:
     * `build/target/`: Rust compiler compilation cache (configured via `.cargo/config.toml`).
     * `build/staging/`: Intermediate extraction, JSON conversion, and container staging (`legacy/`, `json/`, `container/`, `zen/`).
     * `build/dist/`: Production release archive (`CompleteStory-Release.zip`).
4. **Quarantine Portable Binaries into Hidden `.tools/`:**
   * Third-party executable binaries (`retoc.exe`, `UAssetGUI.exe`) and mapping files (`SparkingZERO.usmap`) are quarantined into a dot-prefixed hidden directory (`.tools/`) excluded from Git.
   * `complete-story-cli` auto-discovers these tools and the Steam game directory automatically without requiring configuration files.
5. **Retirement of Legacy Artifacts:**
   * Permanently purged `helpers/` and `config/` from the repository.
   * Superseded ADR 0003 (PowerShell pipeline partitioning) and ADR 0008 (`helpers/` namespace convention).

## Consequences
* **Positive:** Pristine root directory containing only tracked source (`CompleteStory/`, `crates/`, `docs/`, `.agents/`, `.cargo/`) and standard root files (`AGENTS.md`, `README.md`, `Cargo.toml`, `.gitignore`).
* **Positive:** Deterministic, type-safe asset transformations verified by unit tests in milliseconds without invoking heavy external CLI tools.
* **Positive:** Complete elimination of PowerShell array-wrapping bugs during binary asset serialization.
* **Positive:** Single-command build, pack, and deployment workflow (`cargo run -p complete-story-cli -- build --deploy`).
* **Positive:** Integrated live runtime log streaming (`cargo run -p complete-story-cli -- logs`).
