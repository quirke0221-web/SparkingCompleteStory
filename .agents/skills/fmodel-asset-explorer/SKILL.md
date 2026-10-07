---
name: fmodel-asset-explorer
description: Mandatory before exploring, inspecting, or reverse-engineering Dragon Ball Sparking! ZERO packages, Blueprints, DataTables, or audio/texture assets. Governs CUE4Parse, AES decryption keys, SparkingZERO.usmap mapping usage, and asset navigation.
---

# Skill: FModel Asset Explorer

> **Status:** AUDITED & ACTIVE  
> **Pinned Tool:** FModel (CUE4Parse backend)  
> **Target Game:** Dragon Ball: Sparking! ZERO (Unreal Engine 5.1.1)  
> **Reference folder:** [`docs/dependencies/fmodel/`](../../../docs/dependencies/fmodel/README.md)  
> (verbatim upstream receipts: `README.md`, `raw-readme.md`, `sparking-zero-guide.md`)

Every rule below cites its receipt. `[FM]` = upstream FModel documentation, `[SZ]` = `sparking-zero-guide.md`, `[LOCAL]` = local repository assets/configurations.

---

## 1. Scope Boundaries

**Use this skill for:**
- Visually inspecting and exploring Unreal Engine 5.1 IoStore packages (`.utoc`/`.ucas`/`.pak`) without manual CLI unpacking.
- Inspecting Blueprint variable schemas, DataTables, and game settings structures in JSON format.
- Resolving unversioned property tags via `.tools/Data/Mappings/SparkingZERO.usmap`.
- Identifying exact package paths and internal asset names before creating AST transform rules.

**Do NOT use this skill for (hand off instead):**
| Task | Owner skill |
|---|---|
| Unpacking / packing IoStore containers (`.pak`, `.utoc`, `.ucas`) in build pipelines | `retoc-iostore-packer` |
| Binary asset serialization / JSON editing in build pipelines | `uassetgui-asset-serialization` |
| AST manipulation and JSON serialization in Rust | `serde-json-ast` |
| In-game runtime memory inspection and hook execution | `ue4ss-runtime-scripting` / `minhook-native-hooking` |

---

## 2. Core Engine Invariants & Configuration

### 2.1 Engine & Mapping Configuration
- `[SZ]` **UE Version:** Always set FModel's game version to `GAME_UE5_1` (Unreal Engine 5.1.1).
- `[SZ]` **Mapping File (`.usmap`):** Always specify the mappings file at `.tools/Data/Mappings/SparkingZERO.usmap`.
  - Without `.usmap`, unversioned properties serialize as `Property_0`, `Property_1`, obscuring struct and property names.
  - With `.usmap`, all engine properties reflect with their true names (e.g. `PtrRecords`, `DefaultOpenCharacter`, `ClearEventData`).
- `[SZ]` **Game Archive Path:**
  - Target: `<GameRoot>\SparkingZERO\Content\Paks`
- `[POLICY]` **AES Key Handling:**
  - Load AES decryption keys strictly from secure environment variables (e.g. `$env:SPARKING_ZERO_AES_KEY`).
  - Never commit raw AES keys in plain text to version control.

### 2.2 Storage & Anti-Duplication Rules (`AGENTS.md` §0.2, §0.3)
- **Zero Raw Dumps:** Never extract full archive dumps (gigabytes of game files) into the repository working tree.
- **Authoritative Pipeline Extraction:** For assets modified by the build pipeline, extract only the targeted individual packages using `retoc` / `complete-story-cli`.
- Use FModel strictly as an interactive explorer and schema reference.

---

## 3. Verified Golden Paths

### 3.1 Inspecting Episode Battle Schemas
1. In FModel, open archive `pakchunk0-Windows.utoc` (or the relevant chunk).
2. Navigate to `SparkingZERO/Content/SS/Blueprints/`:
   - `DragonAdventureIFData.uasset`: Inspect `PtrRecords` array (character carousel order) and `DefaultOpenCharacter` (default unlocked saga key).
   - `DragonAdventureIFChartData.uasset`: Inspect saga mission graph structure, battle nodes, and branching logic.
3. Export properties to JSON (Ctrl+S or Copy Raw JSON) to inspect the exact AST schema expected by `crates/complete-story-cli/src/transform/`.

### 3.2 Locating Character Story Assets
1. Navigate to `SparkingZERO/Content/SS/MasterDataAsset/DragonAdventureIF/`:
   - Inspect character metadata folders: `0000_00` (custom/template saga), `0000_40` (Goku saga), `0032_00` (Vegeta saga).
2. Cross-reference `CharacterKey` and asset path strings against `complete-story-cli` transform definitions.
