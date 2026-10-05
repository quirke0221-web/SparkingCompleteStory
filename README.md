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
  * `0003`: Separation of Concerns in Asset Pipeline (Orchestrator vs. Pure Transformation)
  * `0004`: Strict Ban on Transient Slate Widget Reflection in Runtime Scripting
  * `0005`: Documentation Taxonomy, Knowledge Consolidation, and Clean Root Standard
  * `0006`: Pruning Legacy Evidence Debris & Distilled Research Standard
  * `0007`: Runtime Scripting Module Consolidation & GameThread Safety
  * `0008`: AccessForge Architectural Parity (Root Mod Folder & Helpers Disambiguation)
* **[Dependency Matrix & Reference Vault](docs/dependencies/dependencies.md):** Pinned tool versions (`retoc`, `UAssetGUI`, `RE-UE4SS`, `Unverum`, `UTOC Bypass`) and source receipts.
* **[Empirical Research Vault](docs/research/):**
  * `asset-data-dictionary.md`: Reverse-engineered `PtrRecords` struct offsets, event pointers, and save structures.
  * `legacy-iteration-history.md`: Complete v0.1–v0.7 failure post-mortems and "Do Not Repeat" matrix.
  * `sparking-zero-modding-ecosystem.md`: Verified community case studies (AccessForge, WistfulHopes).

---

## 2. Quickstart: Building the Mod

Complete Story uses a single, parameterized orchestrator that coordinates `retoc` and `UAssetGUI` across 6 automated stages:

```powershell
# 1. Ensure local config is set in config/project.local.psd1 and AES key is in environment
$env:SPARKING_ZERO_AES_KEY = "0x..."

# 2. Build the complete IoStore container and Unverum release package
.\helpers\Build-CompleteStory.ps1
```

The resulting package is emitted to `dist/CompleteStory-v0.3-Unverum.zip` ready for one-click installation via **Unverum**.

---

## 3. Engineering Guardrails

Contributors and autonomous AI agents (Codex, Antigravity) must strictly obey the governance invariants in [`AGENTS.md`](AGENTS.md) and [`docs/commit.md`](docs/commit.md):
1. **Clean Root Invariant:** Only `.gitignore`, `AGENTS.md`, and `README.md` may reside in the root.
2. **File Length Ceiling:** Strictly **<= 300 lines max per file** across all code and documentation.
3. **No Unapproved Code Changes:** Mandatory implementation planning gate before editing any source or config.
4. **Zero Slate Reflection:** Runtime hooks must never reflect transient Slate/UMG widgets across frames (ADR 0004).
