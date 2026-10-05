# Start Here

Complete Story aims to add a separate thirteenth Episode Battle choice while preserving all twelve original campaigns. Eventually it should orchestrate existing canonical content chronologically across Goku, Piccolo, Gohan, Krillin, Vegeta, and other viewpoints. The first milestone is only: **Complete Story → New Game → Goku's Raditz opening → playable battle**.

## Current truth

- v0.3 is the clean proven asset baseline. It displays Complete Story as a thirteenth choice and preserves the twelve stock choices.
- Complete Story is still locked. Confirmation reaches an unrelated NEO storefront fallback.
- No Complete Story Raditz launch or cross-character transition has been verified.
- v0.7 is a failed diagnostic. Its six logged panel values are relative carousel positions, not registry indexes, and panel reflection crashed Episode Battle.

## Repository map

| Path | Purpose |
|---|---|
| `scripts/` | Transform, import, package, install, and verification helpers |
| `runtime/v0.7/` | Latest failed runtime diagnostic, retained verbatim |
| `archive/legacy/` | Earlier project-authored Lua experiments |
| `archive/release-notes/` | Historical package notes and hashes |
| `docs/` | Design, architecture, build, testing, exclusions, and audit |
| `evidence/` | Sanitized primary evidence and structural summaries |
| `build/`, `work/`, `dist/`, `tools/` | Local-only generated, copyrighted, private, or third-party material |

Read `docs/ARCHITECTURE.md`, then `docs/VERSION_HISTORY.md`. Rebuild instructions are in `docs/BUILD.md`. A legitimate game owner must supply the excluded inputs listed in `docs/LOCAL_INPUTS.md`.

The immediate blocker is identifying stable selected-campaign state and satisfying native playability without reflecting through transient panel widgets. Do not restore the old F8/F9 prototype, change `DefaultOpenCharacter`, edit DLC ownership data, or claim a build works until Raditz is actually observed.
