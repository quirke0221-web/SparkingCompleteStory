# Case Study: WistfulHopes (`SparkingZERO_ModProject`)

> **Subject:** WistfulHopes Community Modding Practices & Tooling Patterns  
> **Domain:** Unreal Engine 5 Zen/IoStore Packaging & Non-Destructive Priority Patching  
> **Significance:** Primary blueprint for Complete Story's container compilation, `_P` patch mounting, and distribution boundaries.

---

## 1. Executive Summary

WistfulHopes is a prominent and prolific modder and reverse-engineer in the anime fighting game community (*Dragon Ball FighterZ*, *Guilty Gear Strive*, *Dragon Ball: Sparking! ZERO*). 

When *Dragon Ball: Sparking! ZERO* launched with Unreal Engine 5.1.1's modern IoStore architecture (`.pak` + `.utoc` + `.ucas`), WistfulHopes' open-source tooling workflows and mod packaging standards demonstrated how custom character models, animations, and UI data tables can be safely compiled and mounted without destabilizing the base game.

---

## 2. Key Architectural Discoveries

### A. The IoStore Zen Packaging Pipeline
* `[FACT]` Unreal Engine 5 uses Zen IoStore containers. Traditional tools (like `UnrealPak.exe`) produce legacy `.pak` files that Sparking! ZERO rejects.
* `[FACT]` Community tool `retoc` by trumank solves this by providing:
  1. `to-legacy`: Converts modern Zen containers to editable `.uasset` / `.uexp` binaries.
  2. `to-zen`: Compiles staged legacy `.uasset` + `.uexp` pairs into modern `.utoc`, `.ucas`, and a sibling `.pak`.
* `[OBSERVATION]` Staging requires both `.uasset` and split `.uexp` export binaries; an un-split `.uasset` is silently skipped by `retoc to-zen`.
* `[POLICY]` Complete Story automates this lifecycle end-to-end in `helpers/Build-CompleteStory.ps1` using pinned `retoc v0.1.5` and `UAssetGUI v1.1.0`.

### B. Non-Destructive `_P` Patch Priority Mounting
* `[FACT]` Unreal Engine's IoStore file system mounts container packages alphabetically and checks for the `_P` ("Patch") naming convention.
* `[OBSERVATION]` When a container is named with `_P` (e.g. `CompleteStory_P.utoc`), the engine's Zen container loader grants it higher lookup priority in memory. Any package path defined in `CompleteStory_P` (such as `/Game/SS/Blueprints/DragonAdventureIFData`) overrides the vanilla table in memory while leaving the original base game files on disk completely untouched.
* `[POLICY]` Guarantees the Non-Destructive Coexistence Invariant (`AGENTS.md` §4.3): If the mod is uninstalled or disabled, the game immediately falls back to stock vanilla behavior with zero file restoration needed.

### C. Clean Distribution Boundaries (ADR 0002)
* `[FACT]` Mod release packages should strictly distribute data containers (`.pak`, `.utoc`, `.ucas`) at the archive root.
* `[OBSERVATION]` Early community modders frequently caused confusion by bundling anti-cheat bypass DLLs (`dsound.dll`) inside their mod zips, risking player bans or conflicts when multiple mods bundled different DLL versions.
* `[POLICY]` Codified as ADR 0002: Complete Story never bundles third-party bypass binaries in release packages. Signature bypass loading and mod load ordering are delegated 100% to **Unverum**.

---

## 3. Direct Impact on Complete Story

| WistfulHopes Modding Pattern | Complete Story Adoption | Settled Decision |
| :--- | :--- | :--- |
| `retoc` Zen container packaging | Automated Stages 1, 5, 6 in `Build-CompleteStory.ps1` | [ADR 0003](../../ADRs/0003-separation-of-concerns-in-asset-pipeline.md) |
| Non-destructive `_P` patch mounting | Outputs `CompleteStory_P.{pak,utoc,ucas}` | [PRD](../../PRD.md) Principle 4 |
| Strict bypass bundling ban | Clean container-only release archives for Unverum | [ADR 0002](../../ADRs/0002-dependency-boundaries-and-unverum-distribution.md) |
| Mod manager delegation | Unverum load ordering and conflict resolution | [ADR 0002](../../ADRs/0002-dependency-boundaries-and-unverum-distribution.md) |
