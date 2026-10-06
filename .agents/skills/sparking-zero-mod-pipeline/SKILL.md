---
name: sparking-zero-mod-pipeline
description: Master orchestrator for the Dragon Ball Sparking! ZERO Complete Story mod engineering lifecycle. Coordinates the multi-step asset pipeline (extract -> serialize -> modify -> compile -> pack -> verify -> package) across all 5 Tier-2 dependency skills while enforcing non-destructive coexistence with stock save data and campaigns.
---

# Skill: Sparking! ZERO Mod Pipeline (Tier 1 Orchestrator)

> **Status:** AUDITED & ACTIVE (Phase 6 Passed)  
> **Target:** *Dragon Ball: Sparking! ZERO* (PC / Steam build `24953175`, UE 5.1.1)  
> **Master Requirements:** [`docs/PRD.md`](../../../docs/PRD.md)  
> **Architecture Single Source of Truth:** [`docs/ARCHITECTURE.md`](../../../docs/ARCHITECTURE.md)  
> **Dependency Matrix:** [`docs/dependencies/dependencies.md`](../../../docs/dependencies/dependencies.md)

This is the **Tier 1 Master Orchestration Skill**. It governs the end-to-end mod build, modification, and verification lifecycle. It does not reinvent tool-specific logic; instead, it coordinates the execution of the 5 atomic Tier-2 dependency skills.

---

## 1. Skill Delegation Architecture

Every tool action in the pipeline is strictly delegated to its dedicated Tier-2 skill:

```text
[TIER 1: sparking-zero-mod-pipeline]
  │
  ├── 1. Build Orchestrator & CLI    ──> rust-pipeline-builder (crates/complete-story-cli)
  ├── 2. Extract IoStore Assets      ──> retoc-iostore-packer
  ├── 3. Serialize .uasset to JSON   ──> uassetgui-asset-serialization
  ├── 4. Modify JSON Schema (AST)    ──> rust-pipeline-builder (transform/)
  ├── 5. Reconstruct Binary Assets   ──> uassetgui-asset-serialization
  ├── 6. Pack Zen IoStore Container  ──> retoc-iostore-packer
  ├── 7. Verify Container Integrity  ──> retoc-iostore-packer
  ├── 8. Package for Distribution    ──> unverum-mod-packager
  └── 9. Runtime Scripting & Hooks   ──> ue4ss-runtime-scripting
```

---

## 2. Non-Destructive Coexistence Invariants (`AGENTS.md` §4.3)

1. **Vanilla Save Data Protection:** Mod modifications must **never** corrupt, invalidate, or overwrite the player's stock save data (`MainGameSaveData`).
2. **Stock Campaign Isolation:** All 12 stock character Episode Battle campaigns (Goku, Vegeta, Gohan, Piccolo, Future Trunks, Frieza, Goku Black, Jiren, etc.) must remain 100% playable and untampered.
3. **Vanilla Asset Reuse:** 100% of audio, cutscenes, and character assets must reuse stock assets from the base game.
4. **Clean Staging Invariant:** Never modify files directly inside the game's installation or `~mods/` directory. All work occurs in `staging/` or `dist/`.

---

## 3. The 6-Stage Mod Engineering Lifecycle

The mod build and packaging lifecycle is coordinated end-to-end by [`crates/complete-story-cli`](file:///c:/echor/projects/SparkingCompleteStory/crates/complete-story-cli) (via `cargo run -p complete-story-cli -- build --deploy`):

### Stage 1: Targeted Extraction
- **Tool:** [`retoc-iostore-packer`](../retoc-iostore-packer/SKILL.md) (`retoc to-legacy --version UE5_1`)
- **Automated by:** `complete-story-cli build` (Stage 1)
- Extracts only the targeted stock asset packages (`DragonAdventureIFData`, `DragonAdventureIFChartData`, character data) and `scriptobjects.bin` using substring `--filter` flags into `staging/legacy/`.

### Stage 2: JSON Deserialization
- **Tool:** [`uassetgui-asset-serialization`](../uassetgui-asset-serialization/SKILL.md) (`UAssetGUI tojson`)
- **Automated by:** `helpers/Build-CompleteStory.ps1` Stage 2
- Converts extracted binary `.uasset` files into indented JSON using `VER_UE5_1` and `SparkingZERO.usmap` into `staging/json/`.

### Stage 3: Pure Domain Transformation
- **Script:** [`helpers/Transform-CompleteStoryAssets.ps1`](file:///c:/echor/projects/SparkingCompleteStory/helpers/Transform-CompleteStoryAssets.ps1)
- **Automated by:** `helpers/Build-CompleteStory.ps1` Stage 3
- Injects campaign entries into the JSON structure (pure in-memory JSON mutation, zero subprocess calls per ADR 0003):
  - Appends `0000_00` to `DragonAdventureIFData.PtrRecords`.
  - Maps `0000_00` in `DragonAdventureIFChartData.PtrRecords`.
  - Clones Goku's asset into `DAIF_CharaData_CompleteStory` with culture-invariant title `"Complete Story"`.

### Stage 4: Binary Asset Recompilation
- **Tool:** [`uassetgui-asset-serialization`](../uassetgui-asset-serialization/SKILL.md) (`UAssetGUI fromjson`)
- **Automated by:** `helpers/Build-CompleteStory.ps1` Stage 4
- Recompiles modified JSON into `.uasset` + companion `.uexp` binaries into `staging/container/`. Copies `scriptobjects.bin`.

### Stage 5: Zen IoStore Container Packaging
- **Tool:** [`retoc-iostore-packer`](../retoc-iostore-packer/SKILL.md) (`retoc to-zen --version UE5_1`)
- **Automated by:** `helpers/Build-CompleteStory.ps1` Stage 5
- Compiles `staging/container/` into `dist/CompleteStory_P.{pak,utoc,ucas}`.

### Stage 6: Verification & Release Packaging
- **Tool:** [`retoc-iostore-packer`](../retoc-iostore-packer/SKILL.md) (`retoc verify`) & [`zip-archive-packager`](../zip-archive-packager/SKILL.md)
- **Automated by:** `complete-story-cli build` (Stage 6)
- Runs `retoc verify` on the compiled `.utoc`.
- Verifies non-zero file sizes across `.pak`, `.utoc`, and `.ucas`.
- Packages the Unverum-ready release archive `dist/CompleteStory-Release.zip` ensuring no third-party bypass DLLs are bundled (ADR 0002).

---

## 4. Pipeline Pre-Flight Checklist

- [ ] Native CLI build toolchain verified (`cargo check -p complete-story-cli`).
- [ ] Dependencies verified in matrix: `retoc`, `UAssetGUI`, `RE-UE4SS`, `Unverum`.
- [ ] `SparkingZERO.usmap` is in place.
- [ ] Target AES key is set in environment (`$env:SPARKING_ZERO_AES_KEY`).
- [ ] Staging and output directories are cleanly isolated outside `~mods/`.

---

## 5. Pipeline Post-Flight Checklist & Diagnostics

- [ ] All 3 container files (`.pak`, `.utoc`, `.ucas`) generated in `dist/`.
- [ ] `retoc verify` outputs `verified`.
- [ ] Staging logs confirm 0 skipped assets (every `.uasset` had its matching `.uexp`).
- [ ] Staged to local game for testing via `helpers/Build-CompleteStory.ps1 -Deploy`.
- [ ] In-game smoke test: Game boots, stock 12 campaigns remain accessible, and custom campaign mounts without crashes.
- [ ] Diagnostic Triage: If an in-game crash, freeze, or exception occurs during testing, run `helpers/Get-ModLogs.ps1` to inspect `ue4ss.log` and Unreal crash dumps directly.

---

## 6. Negative Constraints (Macro Guardrails)

| Forbidden | Why (receipt) | Do instead |
|---|---|---|
| Executing pipeline steps out of order | Downstream tools depend on upstream artifacts | Follow Stages 1 -> 6 linearly |
| Building directly into the game's `~mods/` | Unverum deletes `~mods/` on build click | Build to `dist/`, then deploy |
| Modifying stock save data files directly | Violates `AGENTS.md` §4.3 and PRD Principle 4 | Isolate mod campaign data |
| Bundling third-party bypass DLLs in release | Violates `AGENTS.md` §4.1; Unverum manages bypass | Distribute only `.pak/.utoc/.ucas` |
| Calling tools without checking their Tier-2 skill | Prevents hallucinated flags and argument drift | Always consult Tier-2 skill first |
