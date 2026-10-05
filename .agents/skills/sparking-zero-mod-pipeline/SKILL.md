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
  ├── 1. Extract IoStore Assets      ──> retoc-iostore-packer
  ├── 2. Serialize .uasset to JSON   ──> uassetgui-asset-serialization
  ├── 3. Modify JSON Schema          ──> uassetgui-asset-serialization
  ├── 4. Reconstruct Binary Assets   ──> uassetgui-asset-serialization
  ├── 5. Pack Zen IoStore Container  ──> retoc-iostore-packer
  ├── 6. Verify Container Integrity  ──> retoc-iostore-packer
  ├── 7. Package for Distribution    ──> unverum-mod-packager
  └── 8. Runtime Scripting & Hooks   ──> ue4ss-runtime-scripting
```

---

## 2. Non-Destructive Coexistence Invariants (`AGENTS.md` §4.3)

1. **Vanilla Save Data Protection:** Mod modifications must **never** corrupt, invalidate, or overwrite the player's stock save data (`MainGameSaveData`).
2. **Stock Campaign Isolation:** All 12 stock character Episode Battle campaigns (Goku, Vegeta, Gohan, Piccolo, Future Trunks, Frieza, Goku Black, Jiren, etc.) must remain 100% playable and untampered.
3. **Vanilla Asset Reuse:** 100% of audio, cutscenes, and character assets must reuse stock assets from the base game.
4. **Clean Staging Invariant:** Never modify files directly inside the game's installation or `~mods/` directory. All work occurs in `scratch/`, `staging/`, or `dist/`.

---

## 3. The 6-Stage Mod Engineering Lifecycle

### Stage 1: Targeted Extraction
- **Delegated to:** [`retoc-iostore-packer`](../retoc-iostore-packer/SKILL.md)
- **Script:** [`scripts/Extract-RequiredAssets.ps1`](file:///c:/echor/projects/SparkingCompleteStory/scripts/Extract-RequiredAssets.ps1)
- Extracts only the targeted stock asset packages (`DragonAdventureIFData`, `DragonAdventureIFChartData`, character data) and `scriptobjects.bin` using substring `--filter` flags.

### Stage 2: JSON Deserialization
- **Delegated to:** [`uassetgui-asset-serialization`](../uassetgui-asset-serialization/SKILL.md)
- **Script:** [`scripts/Export-AssetJson.ps1`](file:///c:/echor/projects/SparkingCompleteStory/scripts/Export-AssetJson.ps1)
- Converts extracted binary `.uasset` files into indented JSON using `VER_UE5_1` and `SparkingZERO.usmap`.

### Stage 3: Data Asset Modification & Schema Invariants
- **Delegated to:** [`uassetgui-asset-serialization`](../uassetgui-asset-serialization/SKILL.md)
- Injects the Complete Story campaign entries into the JSON structure:
  - Appends campaign metadata to `DragonAdventureIFData`.
  - Configures canonical battle progression in `DragonAdventureIFChartData`.
  - Links custom character data records.
- **Mandatory Invariant:** Synchronize `"Imports"` table entries whenever external package references are introduced.

### Stage 4: Binary Asset Recompilation
- **Delegated to:** [`uassetgui-asset-serialization`](../uassetgui-asset-serialization/SKILL.md)
- **Script:** [`scripts/Import-ModifiedAssets.ps1`](file:///c:/echor/projects/SparkingCompleteStory/scripts/Import-ModifiedAssets.ps1)
- Recompiles modified JSON into `.uasset` + companion `.uexp` binaries into the staging tree (`staging\SparkingZERO\Content\...`).

### Stage 5: Zen IoStore Container Packaging
- **Delegated to:** [`retoc-iostore-packer`](../retoc-iostore-packer/SKILL.md)
- **Script:** [`scripts/Build-IoStore.ps1`](file:///c:/echor/projects/SparkingCompleteStory/scripts/Build-IoStore.ps1)
- Verifies that `scriptobjects.bin` is staged, and compiles the staged asset tree into `CompleteStory_P.utoc`, `CompleteStory_P.ucas`, and `CompleteStory_P.pak` using `retoc to-zen --version UE5_1`.

### Stage 6: Verification & Release Packaging
- **Delegated to:** [`retoc-iostore-packer`](../retoc-iostore-packer/SKILL.md) & [`unverum-mod-packager`](../unverum-mod-packager/SKILL.md)
- **Script:** [`scripts/Verify-IoStore.ps1`](file:///c:/echor/projects/SparkingCompleteStory/scripts/Verify-IoStore.ps1)
- Runs `retoc verify` on the compiled `.utoc`.
- Verifies non-zero file sizes across all three container files (`.pak`, `.utoc`, `.ucas`).
- Packages the release archive ensuring no third-party bypass (`dsound.dll`, `.asi`) or injection binaries are bundled.

---

## 4. Pipeline Pre-Flight Checklist

- [ ] Local environment configuration exists and is valid (`config/project.local.psd1`).
- [ ] Dependencies verified in matrix: `retoc`, `UAssetGUI`, `RE-UE4SS`, `Unverum`.
- [ ] `SparkingZERO.usmap` is in place.
- [ ] Target AES key is set in environment (`$env:SPARKING_ZERO_AES_KEY`).
- [ ] Staging and output directories are cleanly isolated outside `~mods/`.

---

## 5. Pipeline Post-Flight Checklist

- [ ] All 3 container files (`.pak`, `.utoc`, `.ucas`) generated in `dist/`.
- [ ] `retoc verify` outputs `verified`.
- [ ] Staging logs confirm 0 skipped assets (every `.uasset` had its matching `.uexp`).
- [ ] In-game smoke test: Game boots, stock 12 campaigns remain accessible, and custom campaign mounts without crashes.

---

## 6. Negative Constraints (Macro Guardrails)

| Forbidden | Why (receipt) | Do instead |
|---|---|---|
| Executing pipeline steps out of order | Downstream tools depend on upstream artifacts | Follow Stages 1 -> 6 linearly |
| Building directly into the game's `~mods/` | Unverum deletes `~mods/` on build click | Build to `dist/`, then deploy |
| Modifying stock save data files directly | Violates `AGENTS.md` §4.3 and PRD Principle 4 | Isolate mod campaign data |
| Bundling third-party bypass DLLs in release | Violates `AGENTS.md` §4.1; Unverum manages bypass | Distribute only `.pak/.utoc/.ucas` |
| Calling tools without checking their Tier-2 skill | Prevents hallucinated flags and argument drift | Always consult Tier-2 skill first |
