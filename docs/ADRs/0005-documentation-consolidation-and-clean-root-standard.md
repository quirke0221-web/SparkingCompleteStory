# ADR 0005: Documentation Taxonomy, Knowledge Consolidation, and Clean Root Standard

## Status
Accepted

## Context
During early prototype iterations (v0.1 through v0.7), documentation was authored reactively by developers and coding agents without an architectural taxonomy. This resulted in:
1. **Severe Root Pollution:** 8 sprawling Markdown and CSV files resided in the repository root (`COMPLETE_TECHNICAL_HANDOFF.md`, `FINDINGS.md`, `START_HERE.md`, `PROJECT_STATE.md`, `DEVELOPMENT_HISTORY.md`, `BUILD_REPORT.md`, `ASSET_MAP.md`, `ASSET_INVENTORY.csv`).
2. **Duplicate Failure Chronicles:** The failure history of prototypes v0.1–v0.7 was re-written across 5 separate documents.
3. **Context-Window Splitting for AI Agents:** When an AI agent (such as Codex) indexed the codebase, multiple contradictory files described obsolete build scripts (10 manual scripts purged in Milestone 2) and out-of-date v0.1 statuses, leading to severe hallucination and regressions.
4. **Invariant Violations:** A 31 KB auto-generated file audit (`docs/FILE_AUDIT.md`) violated the repository's 300-line ceiling rule.

## Decision
1. **Enforce the Clean Root Invariant (`AGENTS.md` §0.3):** The repository root must strictly contain only `.gitignore`, `AGENTS.md`, and `README.md`.
2. **Extract-Before-Retire (Zero Knowledge Loss):** All empirical reverse-engineering facts from legacy files were harvested into dedicated research files before retiring any document:
   * Memory layouts, `PtrRecords` `0x40` offset, and start pointers $\rightarrow$ `docs/research/asset-data-dictionary.md`.
   * Complete v0.1–v0.7 failure post-mortems and "Do Not Repeat" matrix $\rightarrow$ `docs/research/legacy-iteration-history.md`.
   * Asset package inventory table $\rightarrow$ `docs/research/asset-inventory.csv`.
3. **Authoritative Operational Docs:**
   * `docs/PRD.md` is the Single Source of Truth for product scope and invariants.
   * `docs/ARCHITECTURE.md` is the authoritative system architecture across the 4 subsystems.
   * `docs/BUILD.md` is the modern runbook documenting `Build-CompleteStory.ps1` and developer workflows.
4. **Formal Retirement of Sprawling Drafts:** The 13 redundant, duplicate, or obsolete files were permanently purged from Git tracking.

## Consequences
* **Positive (Pure Signal for AI Agents):** AI agents (Codex and Antigravity) will no longer hallucinate obsolete scripts or repeat disproven v0.2–v0.7 failure modes. Every query returns 100% authoritative signal.
* **Positive (Workspace Hygiene):** The repository root is pristine, professional, and compliant with repository governance.
* **Preservation Guarantee:** Any future contributor or agent wondering where previous findings went can locate 100% of the technical data in `docs/research/`.
