# Dragon Ball Sparking! ZERO: Complete Story

[![Unreal Engine](https://img.shields.io/badge/Unreal_Engine-5.1.1_IoStore-black?logo=unrealengine)](https://www.unrealengine.com/)
[![Target Build](https://img.shields.io/badge/Steam_Build-24953175-blue)]()
[![Mod Manager](https://img.shields.io/badge/Distribution-Unverum-green)]()

An engineering project introducing an independent, non-destructive 13th Episode Battle campaign featuring **Goku (Mini)** into *Dragon Ball: Sparking! ZERO*, coexisting with the game's 12 stock character campaigns and preserving vanilla save data.

---

## 1. Documentation Taxonomy (Single Source of Truth)

All project documentation follows the strict governance lifecycle defined in [`AGENTS.md`](AGENTS.md):

* **[Product Requirements Document](docs/PRD.md):** Master product requirements, core campaign vision, and non-destructive invariants.
* **[System Architecture](docs/ARCHITECTURE.md):** Subsystem decomposition across the asset build pipeline, runtime interception, packaging, and save coexistence.
* **[Build & Testing Runbook](docs/BUILD.md):** Prerequisites, single-cmdlet build instructions, local developer deployment, and verification protocol.
* **[Git Workflow & Turn-Boundary Cadence](docs/git-workflow.md):** Autonomous commit protocol, turn-boundary hygiene, and rollback procedures.
* **[Commit Standards & Taxonomy](docs/commit.md):** Conventional Commits 1.0.0 rules, header formatting, and architectural scopes.
* **[Architectural Decision Records (ADRs)](docs/ADRs/):** Settled architectural decisions:
  * `0001`: Git as Single Source of Truth & Retirement of Manual Version Folders
  * `0002`: Dependency Boundaries & Delegation to Community Mod Management (Unverum)
  * `0003`: Separation of Concerns in Asset Pipeline (Superseded by `0009`)
  * `0004`: Strict Ban on Transient Slate Widget Reflection in Runtime Scripting
  * `0005`: Documentation Taxonomy, Knowledge Consolidation, and Clean Root Standard
  * `0006`: Pruning Legacy Evidence Debris & Distilled Research Standard
  * `0007`: Runtime Scripting Module Consolidation & GameThread Safety
  * `0008`: AccessForge Architectural Parity (Superseded by `0009`)
  * `0009`: Native Rust CLI Toolchain and Unified Build Sandbox
* **[Dependency Matrix & Reference Vault](docs/dependencies/dependencies.md):** Pinned tool versions (`retoc`, `UAssetGUI`, `RE-UE4SS`, `Unverum`, `UTOC Bypass`) and source receipts.
* **[Empirical Research Vault](docs/research/):**
  * `asset-data-dictionary.md`: Reverse-engineered `PtrRecords` struct offsets, event pointers, and save structures.
  * `legacy-iteration-history.md`: Complete v0.1–v0.7 failure post-mortems and "Do Not Repeat" matrix.
  * **[Case Studies Vault](docs/research/case-studies/):**
    * [`sparking-zero-modding-ecosystem.md`](docs/research/case-studies/sparking-zero-modding-ecosystem.md): Engine architecture and three modding archetypes.
    * [`accessforge.md`](docs/research/case-studies/accessforge.md): In-engine runtime mod architecture, flat module layout, and safe GameThread hooking.
    * [`wistfulhopes.md`](docs/research/case-studies/wistfulhopes.md): Unreal Engine 5 SDK project (`.uproject`) and editor cooking pipeline.
    * [`audio-modding-tool.md`](docs/research/case-studies/audio-modding-tool.md): Headless binary extraction, surgical byte splicing, and IoStore packaging.

---

## 2. Quickstart: Building the Mod

Complete Story uses a single, native CLI orchestrator (`complete-story-cli`) that coordinates `retoc` and `UAssetGUI` across 6 automated stages:

```bash
# 1. Ensure your AES encryption key is set in your environment
$env:SPARKING_ZERO_AES_KEY = "0x..."

# 2. Build the complete IoStore container and deploy directly to game folder:
cargo run -p complete-story-cli -- build --deploy
```

The resulting package is emitted to `build/dist/CompleteStory-Release.zip` ready for one-click installation via **Unverum**, and automatically staged into your local game folder when `--deploy` is used.

---

## 3. Engineering Guardrails

Contributors and autonomous AI agents (Codex, Antigravity) must strictly obey the governance invariants in [`AGENTS.md`](AGENTS.md) and [`docs/commit.md`](docs/commit.md):
1. **Clean Root Invariant:** Only `.gitignore`, `AGENTS.md`, and `README.md` may reside in the root.
2. **Code File Length Ceiling:** Strictly **<= 300 lines max per code file** (`.rs`, `.lua`, etc.). Documentation files (`.md`) are exempt to ensure thoroughness.
3. **Intent Alignment Gate:** Plain-English conversational alignment before editing code; zero technical jargon traps.
4. **Zero Slate Reflection:** Runtime hooks must never reflect transient Slate/UMG widgets across frames (ADR 0004).

---

## 4. How to Vibe-Code With Your Agent (Creator Playbook)

You don't need coding experience or prompt engineering expertise to build mods with this harness. You act as the **Creative Director & Playtester**, while your AI agent (Codex, Antigravity) acts as your **Senior Technical Modder**.

### How to Work With Your Agent:
* **Give Natural Ideas:** Describe what you want in plain conversational English (e.g., *"I want Goku (Mini) to fight Raditz on Planet Namek instead of Earth"*).
* **Confirm the Plan:** Your agent will reply with a plain-English summary of what it's building and what you will test. Simply reply *"Go for it"* or *"Looks good"*.
* **Playtest & Report Back:** Once your agent finishes, it runs `cargo run -p complete-story-cli -- build --deploy` and tells you what to verify in Sparking! ZERO. Launch the game, test the battle, and tell your agent what you observed!
* **Report Bugs in Plain English:** If the game crashes, freezes, or glitches, just tell your agent (e.g., *"It crashed when I defeated Raditz, can you check what happened?"*). Your agent will run `cargo run -p complete-story-cli -- logs`, read the crash stack directly, and fix the issue.

### What the Agent Handles Automatically:
1. **Zero Tool Hallucination:** Uses audited, version-pinned skills (`retoc`, `UAssetGUI`, `RE-UE4SS`, `Unverum`) grounded in live source receipts.
2. **One-Step Build & Deploy:** Rebuilds assets and stages containers and Lua scripts directly into your game with `cargo run -p complete-story-cli -- build --deploy`.
3. **Autonomous Crash Diagnostics:** Diagnoses game crashes and Lua errors directly from `ue4ss.log` and Unreal Engine crash dumps using `cargo run -p complete-story-cli -- logs`.
4. **Save Data Protection:** Guarantees non-destructive coexistence with the 12 vanilla character campaigns and user save data.
5. **Automated Commit Discipline:** Automatically stages, commits, and pushes clean, conventional Git commits at every milestone, keeping the working tree clean and ready.
