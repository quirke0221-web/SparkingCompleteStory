# `UAssetGUI` / `UAssetAPI` Dependency Knowledge Base

* **Tool:** `UAssetGUI` `1.1.0` (commit [`4855f8c`](https://github.com/atenfyr/UAssetGUI/tree/4855f8ceaa724c008a08609e7927db7139007f16))
* **Underlying Engine:** `UAssetAPI` (commit [`bee2f9b`](https://github.com/atenfyr/UAssetAPI/tree/bee2f9b630ac679012c00dde0306eadcc8f21e2b))
* **Author:** atenfyr
* **Repositories:** [atenfyr/UAssetGUI](https://github.com/atenfyr/UAssetGUI) | [atenfyr/UAssetAPI](https://github.com/atenfyr/UAssetAPI)
* **Role in Project:** Headless JSON export/import and binary serialization for cooked Unreal Engine 5.1.1 assets
* **Agent Skill:** `.agents/skills/uassetgui-asset-serialization/SKILL.md`

---

## Reference Documents in this Folder

| File | What it is | Trust level |
|---|---|---|
| [`raw-readme.md`](raw-readme.md) | Upstream UAssetGUI README @ `4855f8c`, verbatim | Primary |
| [`uassetapi-raw-readme.md`](uassetapi-raw-readme.md) | Upstream UAssetAPI README @ `bee2f9b`, verbatim | Primary |
| [`source-receipts.md`](source-receipts.md) | Verbatim source: `Program.cs` CLI parser, `UAGConfig.cs` mappings loader, `UAsset.cs` `.uexp` writing | Primary |
| [`cli-reference.md`](cli-reference.md) | Readable CLI reference, every row line-cited to source receipts | Derived |
| [`sparking-zero-guide.md`](sparking-zero-guide.md) | Project recipes, labeled `[FACT]`/`[LOCAL]`/`[POLICY]`/`[HYPOTHESIS]` | Derived |

All primary files are script-generated from clones at the pinned commits. Do not hand-edit them.
If a derived doc and a primary receipt disagree, the receipt wins.

---

## Correction Log

**2026-10-05:** Source code audit of `UAssetGUI` @ `4855f8c` revealed and corrected several hallucinations:
1. **Dotted Engine Version (`"5.1"`):** `Program.cs` uses `Enum.TryParse` on `EngineVersion`. Dotted strings fail parsing and default to `UNKNOWN`. Must be exact enum name `'VER_UE5_1'`.
2. **Absolute `.usmap` Path:** `TryGetMappings` performs a dictionary lookup by bare filename without extension. Passing an absolute path fails and silently drops mapping support.
3. **Global CLI Flags:** Previous docs claimed `--lang` and `--force_reset_absolutely_everything_permanently` existed. Source inspection confirmed only `portable` / `--portable` is recognized.
4. **Automatic `.uexp` Writing:** Verified true in `UAssetAPI/UAsset.cs` L3138-L3150 (`Write(outputPath)` automatically writes `.uexp` alongside `.uasset` when `UseSeparateBulkDataFiles` is true).

---

## Agent Skill Readiness Status
* **Status:** AUDITED & ACTIVE (`.agents/skills/uassetgui-asset-serialization/SKILL.md`).
