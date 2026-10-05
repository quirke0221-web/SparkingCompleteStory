# `UAssetGUI` Headless CLI Reference

Extracted directly from `atenfyr/UAssetGUI` source code (`UAssetGUI/Program.cs`, `UAGConfig.cs`, and `UAssetAPI/UAsset.cs` @ `4855f8c`).
All line citations reference [`source-receipts.md`](source-receipts.md).

---

## 1. CLI Execution Model

`UAssetGUI.exe` supports two headless subcommands that bypass the Windows Forms graphical interface: `tojson` and `fromjson`.

```console
UAssetGUI.exe [portable|--portable] <command> [arguments...]
```

* `args[0]`: executable path (`UAssetGUI.exe`).
* Optional switch `portable` or `--portable` (if present, sets `UAGConfig.IsPortable = true` and is removed from the arguments list; `Program.cs` L98-L109).
* `args[1]`: subcommand (`tojson` or `fromjson`; `Program.cs` L132-L170).

---

## 2. Subcommands

### A. `tojson` (Binary `.uasset` to JSON Export)

Exports a binary Unreal Engine asset to an indented JSON file.

```console
UAssetGUI.exe tojson <SourcePath> <DestinationPath> <EngineVersion> [MappingsName]
```

* **Argument Positions (`Program.cs` L136-L148):**
  * `<SourcePath>` (`args[2]`): Path to input `.uasset` (or `.umap`).
  * `<DestinationPath>` (`args[3]`): Path to output `.json` file.
  * `<EngineVersion>` (`args[4]`): Target Unreal Engine version.
    * Accepted formats: Exact enum name (e.g. `VER_UE5_1`) parsed via `Enum.TryParse`, OR an integer offset added to `EngineVersion.VER_UE4_0` (e.g. `23` for 4.23).
    * *Notice:* Dotted strings like `"5.1"` are **not** supported by `Enum.TryParse` and will fail to parse (evaluates to `UNKNOWN`).
  * `[MappingsName]` (`args[5]`, Optional, but mandatory for unversioned games like Sparking! ZERO):
    * The bare name of a mapping file located inside `MappingsFolder` (without the `.usmap` extension, e.g. `SparkingZERO`).
    * *Notice:* Passing an absolute path (e.g. `C:\path\to\SparkingZERO.usmap`) **does not work**; `TryGetMappings` performs a dictionary lookup by bare filename key (`UAGConfig.cs` L90-L95, L175-L182).

### B. `fromjson` (JSON to Binary `.uasset` Import)

Reconstructs a binary `.uasset` and companion `.uexp` from a modified JSON file.

```console
UAssetGUI.exe fromjson <SourcePath> <DestinationPath> [MappingsName]
```

* **Argument Positions (`Program.cs` L151-L170):**
  * `<SourcePath>` (`args[2]`): Path to modified `.json` file.
  * `<DestinationPath>` (`args[3]`): Path to destination `.uasset` file.
  * `[MappingsName]` (`args[4]`, Optional): Bare name of the `.usmap` file in `MappingsFolder` (e.g. `SparkingZERO`).
* *Engine Version:* Not required as a CLI parameter because engine version metadata is deserialized directly from the JSON header.
* *Companion `.uexp` Generation:* `UAsset.Write(outputPath)` automatically writes both the `.uasset` file and the companion `.uexp` file (`Path.ChangeExtension(outputPath, "uexp")`) whenever `UseSeparateBulkDataFiles` is true and `Exports.Count > 0` (`UAsset.cs` L3138-L3150).

---

## 3. Mappings Folder Resolution

The mappings folder (`UAGConfig.MappingsFolder`, `UAGConfig.cs` L77) resolves as follows:
* **Portable Mode (`portable` flag or `Data\config.json` containing `"UAssetGUI"`):**
  `<ExeDirectory>\Data\Mappings\`
* **Standard Mode:**
  `%LOCALAPPDATA%\UAssetGUI\Mappings\`

All `.usmap` files inside that directory are indexed into `AllMappings` with their filename without extension as the key (`UAGConfig.cs` L90-L95).
