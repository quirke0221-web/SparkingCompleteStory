# ADR 0007: Runtime Scripting Module Consolidation & GameThread Safety

## Status
Accepted

## Context
During early prototype iterations (v0.4 through v0.7), the runtime scripting layer suffered from directory fragmentation, duplicate codebases, and unsafe memory reflection:
1. **Directory Naming Schism:** Developers diverged between `mods/CompleteStory13thTest` (matching the UE4SS install folder name) and `runtime/CompleteStory/` / `runtime/v0.7/` (matching repository domain categorization). This produced three conflicting `main.lua` files.
2. **Re-Inventing the Wheel & Slate Reflection Crashes:** In v0.6 and v0.7, scripts attempted to scrape screen text via `FindAllOf("TextBlock")` and read carousel panel properties. Because Slate UI widgets are transient and pooled in Unreal Engine 5, dereferencing them inside native callbacks caused fatal `0xC0000005` access violations (violating ADR 0004).
3. **Manual Deployment Friction:** `Deploy-DevelopmentBuild.ps1` only deployed container assets to `~mods/`, forcing developers to manually copy-paste runtime scripts into `Binaries\Win64\Mods`.

## Decision
1. **Single Authoritative Runtime Module:** Permanently retire `mods/` and `runtime/v0.7/`. Declare `runtime/CompleteStory/` (containing `enabled.txt` and `scripts/main.lua`) as the sole authoritative UE4SS runtime mod in the repository.
2. **Zero Slate/UMG Scraping (ADR 0004 Enforcement):** Strictly ban `FindAllOf("TextBlock")`, UI text normalization, and carousel panel reflection. The runtime script is a clean, GameThread-safe hook on `/Script/SS.SSDragonAdventureIFCSManager:IsPlayable`.
3. **Safe Native Inspection:** Inspect native C++ manager properties on `self` safely wrapped in `pcall` without touching the widget hierarchy.
4. **Non-Destructive Coexistence Invariant:** Override `ReturnValue` to `true` strictly when the active route key resolves to `0000_00` (Complete Story). Return `nil` for all 12 stock character campaigns so vanilla save validation runs untouched.
5. **Dual Development Deployment:** Enhance `Deploy-DevelopmentBuild.ps1` so `-Install` stages both the IoStore container (`Content/Paks/~mods/CompleteStory`) and the RE-UE4SS runtime mod (`Binaries/Win64/Mods/CompleteStory`) with rollback snapshots.

## Consequences
* **Positive:** Eliminates ~760 lines of duplicate, obsolete, and crash-prone code across 3 legacy implementations.
* **Positive:** Single, authoritative source of truth for runtime scripting. Autonomous agents (Codex, Antigravity) cannot be confused by competing mod directories.
* **Positive:** Complete elimination of dangling Slate widget crashes.
* **Positive:** One-command developer deployment and rollback for both asset containers and runtime scripts.
