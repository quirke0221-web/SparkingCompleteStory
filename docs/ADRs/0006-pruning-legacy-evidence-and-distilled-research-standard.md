# ADR 0006: Pruning Legacy Evidence Debris & Distilled Research Standard

## Status
Accepted

## Context
During early reverse engineering, the repository accumulated loose directories `evidence/` and `history/` containing:
1. Raw memory dump greps (`episode-battle-symbols.txt`, `targeted-symbols.txt`).
2. Raw runtime crash traces and test inis from failed experiments (`v0.7-full-sanitized.log`, `UE4SS-settings-test.ini`).
3. Hashes of deleted local zip files (`SHA256SUMS.txt`) and redundant 3-line filter lists (`required-assets.txt`).
4. Machine-readable JSON diffs (`history/v0.1`–`v0.5/asset-delta.json`) created before the build pipeline was automated.
5. Redundant CSV spreadsheets (`asset-inventory.csv`) and temporary session scratchpads (`agent-legacy-audit.md`).

While these files represented the incremental steps of earlier exploration, they were symptoms of pre-automation workflows. Once the build orchestrator (`Build-CompleteStory.ps1`), transformation tool (`Transform-CompleteStoryAssets.ps1`), and formal ADRs (0001–0005) were established, retaining raw debris became an anti-pattern. Leaving raw logs and obsolete deltas in the tree creates cognitive noise, wastes AI agent context tokens, and risks regression.

## Decision
1. **Purge the Raw Debris:** Permanently remove `evidence/`, `history/`, and redundant research scratchpads from the repository.
2. **Distilled Research Standard:** Maintain strictly three high-signal, authoritative research documents in `docs/research/`:
   * `asset-data-dictionary.md`: Active technical blueprint containing verified C++ symbols, struct offsets (`0x40`), save structures, and Raditz start pointers.
   * `legacy-iteration-history.md`: Anti-regression shield documenting what failed in v0.1–v0.7 and the "Do Not Repeat" matrix.
   * `case-studies/`: Modular community case studies (`sparking-zero-modding-ecosystem.md`, `accessforge.md`, `wistfulhopes.md`) proving safe UE4SS hooking and IoStore patching.
3. **Zero Digital Hoarding Invariant:** Do not hoard intermediate test logs, raw memory dumps, or obsolete file hashes in Git when their insights have already been codified into production code, skills, or settled ADRs.

## Consequences
* **Positive (Context Window Hygiene):** Eliminates 18 files and over 1,300 lines of dead text. AI agents (Codex, Antigravity) are presented with 100% actionable signal.
* **Positive (Zero Knowledge Loss):** Every actionable symbol name, memory offset, and anti-pattern lesson is preserved in structured, maintainable documentation.
* **Positive (Clean Workspace):** Removes loose `evidence/` and `history/` directories from the repository root.
