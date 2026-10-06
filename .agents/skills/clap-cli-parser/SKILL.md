---
name: clap-cli-parser
description: Mandatory before authoring or modifying CLI argument parsing or subcommands in crates/complete-story-cli. Governs clap 4.6.7 derive macros and command structure.
---

# Skill: Clap CLI Parser (`clap-cli-parser`)

> **Status:** AUDITED & ACTIVE  
> **Crate Baseline:** `clap 4.6.7` (features: `["derive"]`)  
> **Reference Folder:** [`docs/dependencies/clap/`](../../../docs/dependencies/clap/README.md)  
> **Primary Source:** [`clap-rs/clap` v4.6.7](https://github.com/clap-rs/clap/tree/v4.6.7)  

Every rule below cites its empirical receipt. If an attribute or syntax pattern is not cited here or in `docs/dependencies/clap/`, it is not verified. Do not invent it (see `AGENTS.md` §1).

---

## 1. Scope Boundaries

**Use this skill for:**
* Defining command-line flags, options, arguments, and subcommands in `crates/complete-story-cli`.
* Using `#[derive(Parser, Subcommand, Args)]` macros.
* Configuring CLI help strings, defaults, and version information.

**Do NOT use this skill for:**
* Subprocess execution or exit code verification (delegate to `anyhow-error-handling`).
* AST transformation logic (delegate to `serde-json-ast`).

---

## 2. Verified Golden Paths (Clap 4.6.7)

### 2.1 Crate Dependency Configuration
In `crates/complete-story-cli/Cargo.toml`:
```toml
[dependencies]
clap = { version = "4.6.7", features = ["derive"] }
```

### 2.2 Subcommand Definition Pattern
From `docs/dependencies/clap/derive-receipts.md` (`examples/tutorial_derive/03_04_subcommands.rs`):

```rust
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "complete-story-cli",
    version,
    about = "Dragon Ball Sparking! ZERO Complete Story Mod Orchestrator",
    long_about = None,
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Build mod assets, pack container, and optionally deploy
    Build(BuildArgs),

    /// Pack staged assets into Zen IoStore container
    Pack,

    /// Deploy built mod to Steam ~mods and UE4SS mods folders
    Deploy,

    /// Stream live runtime logs from UE4SS
    Logs,
}

#[derive(Args, Debug)]
pub struct BuildArgs {
    /// Skip Stage 1 extraction if staging/legacy is already populated
    #[arg(long)]
    pub skip_extract: bool,

    /// Automatically deploy built containers to Steam ~mods directory
    #[arg(long)]
    pub deploy: bool,
}
```

### 2.3 Invocation & Matching
In `crates/complete-story-cli/src/main.rs`:
```rust
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build(args) => run_build(args)?,
        Commands::Pack => run_pack()?,
        Commands::Deploy => run_deploy()?,
        Commands::Logs => run_logs()?,
    }
    Ok(())
}
```

---

## 3. Negative Constraints (Hallucination Defense)

| Forbidden | Why (Receipt) | Do Instead |
|---|---|---|
| Using `clap 2.x/3.x` macros (e.g., `clap_app!`, `app_from_crate!`) | Completely removed in clap 4.x | Use `#[derive(Parser, Subcommand)]` |
| Omitting `features = ["derive"]` in `Cargo.toml` | Compilation fails: `cannot find derive macro 'Parser'` | Explicitly specify `features = ["derive"]` |
| Using stringly typed arg queries (e.g. `matches.value_of("arg")`) | Loses compile-time type safety | Use strongly typed fields on `Args` structs |
| Writing CLI parsing code exceeding 300 lines | Violates repo hygiene rule (`AGENTS.md` §0.4) | Keep CLI parsing cleanly isolated in `main.rs` |
