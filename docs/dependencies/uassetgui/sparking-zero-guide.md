# `UAssetGUI` / `UAssetAPI` Implementation Guide for Dragon Ball: Sparking! ZERO

**Target Version:** `UAssetGUI 1.1.0` (commit [`4855f8c`](https://github.com/atenfyr/UAssetGUI/tree/4855f8ceaa724c008a08609e7927db7139007f16))  
**Engine Version:** Unreal Engine 5.1.1 (`VER_UE5_1`)  
**Mapping Name:** `SparkingZERO` (file: `SparkingZERO.usmap`, SHA-256 `B7AE00F54BA558EF7793CABA3437B3E82D64D29A6822C34CE73B4A1F91B2D1C5`)  

Labels: `[FACT]` = source receipt, `[LOCAL]` = local project evidence, `[POLICY]` = project rule, `[HYPOTHESIS]` = unproven.

---

## 1. Exporting Assets to JSON (`tojson`)

- `[FACT]` Cooked assets in Sparking! ZERO use unversioned properties. Property names are stripped from the binary headers. Without a `.usmap` mapping file, property names cannot be resolved and are serialized as uninterpretable raw hex byte arrays.
- `[FACT]` The CLI command syntax (`source-receipts.md` §1):
  ```powershell
  & $uassetGui tojson $sourceAsset $outputJson 'VER_UE5_1' $mappingName
  ```
- `[FACT]` The engine version argument **must** be the exact enum string `'VER_UE5_1'`. Dotted string `'5.1'` fails C# enum parsing and falls back to `UNKNOWN`.
- `[FACT]` `$mappingName` must be the bare name without extension (`'SparkingZERO'`). The file `SparkingZERO.usmap` must exist inside `Data\Mappings\` (portable) or `%LOCALAPPDATA%\UAssetGUI\Mappings\`.
- `[LOCAL]` Implemented authoritatively in `crates/complete-story-cli/src/serialize.rs` (Stage 2).

---

## 2. Reconstructing Binary Assets (`fromjson`)

- `[FACT]` The CLI command syntax (`source-receipts.md` §1):
  ```powershell
  & $uassetGui fromjson $sourceJson $destinationAsset $mappingName
  ```
- `[FACT]` `fromjson` does not accept an `<EngineVersion>` argument because the engine version and custom versions are deserialized directly from the JSON header.
- `[FACT]` Companion `.uexp` writing: `UAsset.Write(outputPath)` automatically writes both `.uasset` and `.uexp` when `UseSeparateBulkDataFiles` is true and `Exports.Count > 0` (`source-receipts.md` §5).
- `[LOCAL]` Implemented authoritatively in `crates/complete-story-cli/src/container.rs` (Stage 4).

---

## 3. The UAssetAPI JSON Schema Mental Model

Exported JSON contains three primary top-level arrays:

```text
{
  "Exports": [ ... ],   // Serialized UObject instances contained in this package
  "Imports": [ ... ],   // External packages, classes, and structs referenced by Exports
  "NameMap": [ ... ]    // Array of FNames referenced in this package
}
```

### A. Working with `Exports`
- In data tables like `DragonAdventureIFData.uasset`, `Exports[0]` holds the primary serialized property table under the `"Data"` array:
  ```json
  "Data": [
    {
      "Name": "PtrRecords",
      "Value": [ ... ]
    },
    {
      "Name": "DefaultOpenCharacter",
      "Value": [ ... ]
    }
  ]
  ```

### B. The `Imports` Invariant (Anti-Corruption Law)
- `[POLICY]` When modifying an asset to reference an external package (such as pointing `DragonAdventureIFData` to our custom character data asset `/Game/SS/MasterDataAsset/DragonAdventureIF/CompleteStory/DAIF_CharaData_CompleteStory`):
  1. An external reference entry **MUST** exist in the `Imports` array.
  2. The export property's `Value` must point to the negative 1-based index corresponding to that import entry.
  3. Omitting the import entry corrupts the asset's package reference table, causing Unreal Engine's package linker to crash or reject the asset during level load.

---

## 4. Hallucination Prevention & Common Traps

| Trap | What Happens | Empirical Fact & Correct Procedure |
|---|---|---|
| **Passing dotted version `'5.1'`** | `Enum.TryParse` fails; engine version defaults to `UNKNOWN`. | `[FACT]` Always pass exact enum identifier `'VER_UE5_1'`. |
| **Passing full path to `.usmap`** | `AllMappings.TryGetValue` fails to find key; mappings are silently ignored. | `[FACT]` Place `SparkingZERO.usmap` in `<ExeDir>\Data\Mappings\` and pass bare name `'SparkingZERO'`. |
| **Manually trying to generate `.uexp`** | Redundant or conflicting file creation. | `[FACT]` `fromjson` generates `.uexp` automatically alongside `.uasset`. |
| **Passing `<EngineVersion>` to `fromjson`** | Misaligns argument positions (`args[4]` treated as mapping name). | `[FACT]` `fromjson` takes only 3 positional arguments: `<source.json>`, `<dest.uasset>`, `[mapping]`. |
