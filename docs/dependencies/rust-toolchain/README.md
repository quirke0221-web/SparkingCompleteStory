# Rust Toolchain Compiler & Runtime Baseline

* **Toolchain Baseline:** `rustc 1.98.1` / `cargo 1.98.1` (`x86_64-pc-windows-msvc`)
* **Target Crate:** `crates/complete-story-cli`
* **Status:** AUDITED & PINNED
* **Agent Skill:** [`.agents/skills/rust-pipeline-builder/`](../../../.agents/skills/rust-pipeline-builder/SKILL.md)

---

## 1. Toolchain Role & Scope

The Rust toolchain provides the native compilation environment for `complete-story-cli`. It replaces legacy Windows PowerShell build automation with a strongly-typed native CLI binary.

---

## 2. Skill-Gated Crate Knowledge Bases

Per **`AGENTS.md` §1.4 (The Skill-Gated Dependency Law)**, every external crate library imported by `complete-story-cli` has its own dedicated audit, reference vault, and agent skill:

| Crate | Verified Version | Features | Reference Vault | Dedicated Agent Skill |
| :--- | :--- | :--- | :--- | :--- |
| **`clap`** | `4.6.7` | `["derive"]` | [`docs/dependencies/clap/`](../clap/README.md) | [`.agents/skills/clap-cli-parser/`](../../../.agents/skills/clap-cli-parser/SKILL.md) |
| **`serde`** | `1.0.229` | `["derive"]` | [`docs/dependencies/serde-json/`](../serde-json/README.md) | [`.agents/skills/serde-json-ast/`](../../../.agents/skills/serde-json-ast/SKILL.md) |
| **`serde_json`** | `1.0.151` | `["preserve_order"]` | [`docs/dependencies/serde-json/`](../serde-json/README.md) | [`.agents/skills/serde-json-ast/`](../../../.agents/skills/serde-json-ast/SKILL.md) |
| **`anyhow`** | `1.0.104` | default (`std`) | [`docs/dependencies/anyhow/`](../anyhow/README.md) | [`.agents/skills/anyhow-error-handling/`](../../../.agents/skills/anyhow-error-handling/SKILL.md) |
| **`zip`** | `8.6.0` | `default-features = false`, `["deflate"]` | [`docs/dependencies/zip/`](../zip/README.md) | [`.agents/skills/zip-archive-packager/`](../../../.agents/skills/zip-archive-packager/SKILL.md) |

---

## 3. Toolchain Invariants

1. **Exact Local Baseline:** Local compiler is `rustc 1.98.1` / `cargo 1.98.1`.
2. **Environment-Sourced AES Key:** Never hardcode the AES key; always read from `std::env::var("SPARKING_ZERO_AES_KEY")`.
3. **No Unvetted Crates:** No crate may be added to `Cargo.toml` without its dedicated entry in `docs/dependencies/dependencies.md`, dedicated reference vault, and dedicated agent skill.
