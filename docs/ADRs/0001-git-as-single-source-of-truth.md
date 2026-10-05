# ADR 0001: Git as the Single Source of Truth & Retirement of Manual Version Folders

## Status
Accepted

## Context
During early prototype iterations (v0.1 through v0.7), the codebase accumulated numerous manual snapshot folders, backup trees, and timestamped directories (e.g., `history/v0.1-v0.5`, `archive/legacy/`, `runtime/v0.7`). This practice led to:
1. Severe code drift and confusion about which script or asset was authoritative.
2. Accidental regressions when an agent referenced stale logic from an archive folder.
3. Bloated repository size and clutter across the workspace.

## Decision
1. **Single Source of Truth:** Git is declared the sole, authoritative version control system for Complete Story.
2. **Zero Working Tree Clones:** Creating manual version-suffixed folders, timestamped directories, or backup subtrees in the workspace is **strictly FORBIDDEN**.
3. **Historical Provenance via Git:** All versioning, experimental branching, and historical provenance must be managed exclusively through Git commits, branches, and tags.
4. **Clean Retirement:** Obsolete, experimental, or superseded code must be removed or refactored via Git commits rather than archived into parallel working directories.

## Consequences
* **Positive:** The repository root and project tree remain clean and navigable. AI agents (Codex, Antigravity) cannot accidentally ingest obsolete files or hallucinate based on stale snapshots.
* **Positive:** Full diff visibility and rollback capabilities are preserved through standard Git operations (`git log`, `git checkout`, `git revert`).
* **Negative / Compliance Requirement:** Human contributors and autonomous agents must commit frequently with clear Conventional Commit messages rather than stashing copies on disk.
