# ADR 0008: AccessForge Architectural Parity (Root Mod Folder & Helpers Disambiguation)

## Status
Superseded by [ADR 0009](0009-native-rust-cli-and-unified-build-sandbox.md) (Native Rust CLI and Unified Build Sandbox)

## Context
Following the consolidation of legacy prototypes in ADR 0007, two architectural friction points remained:
1. **Unnecessary Wrapper Nesting:** The authoritative runtime mod was nested under an artificial `runtime/` directory (`runtime/CompleteStory/`), which added cognitive overhead and diverged from community standards.
2. **Semantic "Scripts" Collision:** The repository contained two completely unrelated folders named `scripts`:
   * Root `scripts/`: PowerShell build and deployment tools executing on Windows outside the game.
   * Mod `scripts/` (`runtime/CompleteStory/scripts/`): Lua code executing inside Unreal Engine 5 memory via RE-UE4SS during gameplay.
   This collision caused persistent confusion regarding which runtime environment executed which code.

Audit of the primary community case study, **[AccessForge (`SparkingZeroAccess`)](../research/case-studies/accessforge.md)**, revealed how experienced mod authors structure hybrid UE4SS projects:
* The mod folder lives directly at the root (`SparkingZeroAccess/`), matching the exact directory dropped into `Win64\Mods\`.
* External build, packaging, and data scripts live in `helpers/`, completely avoiding any collision with in-game Lua scripts.

## Decision
1. **Direct Mod Root (`CompleteStory/`):** Permanently eliminate the `runtime/` wrapper. The mod source lives directly at the repository root as `CompleteStory/`, establishing a bit-for-bit 1:1 match with the deployed game folder `SparkingZERO\Binaries\Win64\Mods\CompleteStory`.
2. **Namespace Disambiguation via `helpers/`:** Rename root `scripts/` to `helpers/`. This strictly formalizes the two execution domains:
   * **`helpers/`**: PowerShell automation for developers and autonomous agents to extract stock assets, transform JSON registries, compile containers, and stage local builds.
   * **`CompleteStory/scripts/`**: In-game Lua scripts loaded into live memory by UE4SS during gameplay.
3. **Pipeline Path Synchronization:** Update `helpers/Deploy-DevelopmentBuild.ps1`, `docs/BUILD.md`, and `docs/ARCHITECTURE.md` to reference the authoritative `helpers/` and `CompleteStory/` paths.

## Consequences
* **Positive:** Complete architectural alignment with the battle-tested AccessForge community case study.
* **Positive:** 1:1 mental model between the Git repository and the `Win64\Mods\CompleteStory` runtime installation.
* **Positive:** Total elimination of the `scripts/` vs `scripts/` naming collision. Developers and AI coding assistants immediately know that `helpers/` is PowerShell build automation and `CompleteStory/scripts/` is live game Lua.
