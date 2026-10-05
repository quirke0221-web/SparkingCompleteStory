---
name: uassetgui-asset-serialization
description: Mandatory before ANY UAssetGUI invocation. Use when converting Dragon Ball Sparking! ZERO .uasset binaries to readable JSON (tojson) or reconstructing modified JSON back into .uasset/.uexp binaries (fromjson).
---

# Skill: UAssetGUI Asset Serialization

> **Status:** AUDITED & ACTIVE (Phase 2 Passed)  
> **Pinned version:** UAssetGUI `1.1.0` (commit [`4855f8c`](https://github.com/atenfyr/UAssetGUI/tree/4855f8ceaa724c008a08609e7927db7139007f16))  
> **Underlying library:** UAssetAPI (commit [`bee2f9b`](https://github.com/atenfyr/UAssetAPI/tree/bee2f9b630ac679012c00dde0306eadcc8f21e2b))  
> **Reference folder:** [`docs/dependencies/uassetgui/`](../../../docs/dependencies/uassetgui/README.md)  
> (verbatim source receipts: `source-receipts.md`, `cli-reference.md`, `sparking-zero-guide.md`)

Every rule below cites its empirical receipt. `[P:Lx]` = line `x` of `UAssetGUI/Program.cs`, `[C:Lx]` = line `x` of `UAssetGUI/UAGConfig.cs`, `[A:Lx]` = line `x` of `UAssetAPI/UAssetAPI/UAsset.cs`. `[LOCAL]` = this repo's validated scripts. If a behavior is not cited here, it is not verified. Do not invent it (see `AGENTS.md` §1).

---

## 1. Scope Boundaries

**Use this skill for:**
- `tojson`: Headless export of binary `.uasset` / `.umap` to indented JSON.
- `fromjson`: Headless compilation of modified JSON into `.uasset` and companion `.uexp`.
- Schema invariants: Managing `Exports`, `Imports`, and `NameMap` in serialized JSON.

**Do NOT use this skill for (hand off instead):**
| Task | Owner skill |
|---|---|
| Unpacking / packing IoStore containers (`.pak`, `.utoc`, `.ucas`) | `retoc-iostore-packer` |
| Container staging, `_P` naming, and `~mods/` deployment | `unverum-mod-packager` |
| Runtime Lua hooks & memory patching | `ue4ss-runtime-scripting` |
| End-to-end build orchestration | `sparking-zero-mod-pipeline` |

---

## 2. Verified Commands (Golden Paths)

### 2.1 Export to JSON: `tojson`

```powershell
# $uassetgui = configured UAssetGUIPath (config/project.local.psd1)
# $source    = path to extracted .uasset
# $dest      = path to output .json
# $mapping   = bare name of mapping file, e.g. 'SparkingZERO'
& $uassetgui tojson $source $dest 'VER_UE5_1' $mapping
if ($LASTEXITCODE -ne 0) { throw 'UAssetGUI tojson failed' }
```

Receipt-backed facts:
- Argument order is strictly: `<source> <destination> <engine_version> [mappings_name]` `[P:L134-L148]`.
- `<engine_version>` (`args[4]`): Parsed via `Enum.TryParse(args[4], out selectedVer)` `[P:L144]`. Must be the exact enum string `'VER_UE5_1'`. Dotted strings like `'5.1'` fail C# enum parsing and fall back to `UNKNOWN`.
- `[mappings_name]` (`args[5]`): Looked up via `UAGConfig.TryGetMappings(args[5])` `[P:L140]`. Looks up the dictionary key `Path.GetFileNameWithoutExtension(mappingPath)` in `MappingsFolder` `[C:L90-L95, L177]`. Must be the bare name without extension (`'SparkingZERO'`). Passing an absolute path fails the dictionary lookup and drops mapping support.
- Local implementation: [`helpers/Build-CompleteStory.ps1`](file:///c:/echor/projects/SparkingCompleteStory/helpers/Build-CompleteStory.ps1#L64) line 64.

### 2.2 Reconstruct Binary Assets: `fromjson`

```powershell
& $uassetgui fromjson $sourceJson $destUasset $mapping
if ($LASTEXITCODE -ne 0) { throw 'UAssetGUI fromjson failed' }
```

Receipt-backed facts:
- Argument order is strictly: `<source_json> <dest_uasset> [mappings_name]` `[P:L149-L170]`.
- Notice: `fromjson` takes **no engine version parameter**. Positional index `args[4]` is the mapping name. Passing an engine version shifts argument positions and breaks mapping loading.
- Companion `.uexp` writing: `UAsset.Write(outputPath)` automatically checks `if (this.UseSeparateBulkDataFiles && this.Exports.Count > 0)` and writes `.uexp` alongside `.uasset` `[A:L3138-L3150]`. Cooked Sparking! ZERO assets have `UseSeparateBulkDataFiles = true` preserved from the original asset header `[A:L569]`.
- Local implementation: [`helpers/Build-CompleteStory.ps1`](file:///c:/echor/projects/SparkingCompleteStory/helpers/Build-CompleteStory.ps1#L90) line 90.

---

## 3. Schema Invariants (JSON Modification Rules)

When programmatically editing exported JSON:
1. **The `Imports` Table Invariant:** If adding an export property that references an external package (e.g. `/Game/SS/MasterDataAsset/.../DAIF_CharaData_CompleteStory`), you **MUST** insert an entry into the top-level `"Imports"` array. Referencing an external package without an `Imports` entry corrupts the asset header and crashes Unreal Engine's package linker.
2. **Negative Import Indices:** Export references to imports must point to the 1-based negative index in `"Imports"` (e.g. `"-1"` points to `Imports[0]`).
3. **Preserve `UseSeparateBulkDataFiles`:** Do not alter the top-level `"UseSeparateBulkDataFiles": true` property in the JSON, as it controls companion `.uexp` generation on write.

---

## 4. Pre-Flight Checklist (run BEFORE invoking)

- [ ] Path check: UAssetGUI binary exists at `$config.UAssetGUIPath` `[LOCAL: helpers/Common.ps1:10]`.
- [ ] Mapping location check: `SparkingZERO.usmap` exists at `<ExeDir>\Data\Mappings\SparkingZERO.usmap` (if portable mode) or `%LOCALAPPDATA%\UAssetGUI\Mappings\SparkingZERO.usmap` `[LOCAL: helpers/Common.ps1:11]`.
- [ ] Argument check for `tojson`: Engine version parameter is strictly string `'VER_UE5_1'`, not `'5.1'` or `'UE5_1'`.
- [ ] Argument check for `fromjson`: Exactly 3 positional arguments passed (`source.json`, `dest.uasset`, `'SparkingZERO'`). No engine version.
- [ ] Destination directory for binary output exists before calling `fromjson` `[LOCAL: helpers/Build-CompleteStory.ps1:89]`.

---

## 5. Post-Flight Checklist (run AFTER invoking)

- [ ] Exit code is `0`.
- [ ] **tojson:** Destination JSON file exists and size is > 0.
- [ ] **tojson:** Inspect output JSON to ensure properties are not serialized as raw hex arrays (confirms `SparkingZERO.usmap` successfully bound).
- [ ] **fromjson:** Both `.uasset` AND companion `.uexp` exist at the destination directory.
- [ ] **fromjson:** Verify `.uasset` and `.uexp` file sizes are both > 0 bytes.

---

## 6. Negative Constraints (Hallucination Defense)

| Forbidden | Why (receipt) | Do instead |
|---|---|---|
| Passing dotted version `'5.1'` to `tojson` | `Enum.TryParse` fails, defaults to `UNKNOWN` `[P:L144]` | Pass `'VER_UE5_1'` |
| Passing full path to `.usmap` | `AllMappings.TryGetValue` looks up bare key `[C:L90-95, L177]` | Pass bare name `'SparkingZERO'` |
| Passing `<EngineVersion>` to `fromjson` | `fromjson` expects `[mappings name]` at `args[4]` `[P:L155]` | Omit engine version from `fromjson` |
| Manually trying to split/generate `.uexp` | `UAsset.Write` automatically creates companion `.uexp` `[A:L3138-L3150]` | Let UAssetGUI write both |
| Adding external package references without `Imports` entry | Package linker validation fails at game launch | Add corresponding `Imports` record |
| Inventing CLI flags like `--lang` or `--force_reset` | Not recognized by `Program.cs` `[P:L98-L109]` | Use only `portable` or standard subcommands |
