# `clap` 4.6.7 Derive Code Receipts

> **Primary Source:** [`clap-rs/clap` v4.6.7](https://github.com/clap-rs/clap/tree/v4.6.7)  
> **Receipt File:** `examples/tutorial_derive/03_04_subcommands.rs`  
> **Retrieved:** 2026-10-06 live via GitHub API / raw content  

---

## 1. Verbatim Subcommands Receipt (`03_04_subcommands.rs`)

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add { name: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add { name } => {
            println!("'myapp add' was used, name is: {name:?}");
        }
    }
}
```

---

## 2. Flags & Optional Arguments Receipt (`03_01_flag_bool.rs`)

```rust
use clap::{Args, Parser};

#[derive(Args)]
pub struct BuildArgs {
    /// Skip asset extraction if build/staging/legacy is already populated
    #[arg(long)]
    pub skip_extract: bool,

    /// Stage built containers directly to Steam ~mods folder
    #[arg(long)]
    pub deploy: bool,
}
```

---

## 3. Mandatory Crate Configuration (`Cargo.toml`)

To use `#[derive(Parser, Subcommand, Args)]`, the `derive` feature must be explicitly enabled:

```toml
[dependencies]
clap = { version = "4.6.7", features = ["derive"] }
```
