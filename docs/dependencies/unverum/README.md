# `Unverum` Dependency Knowledge Base

* **Tool:** `Unverum`, audited at commit [`e41b135`](https://github.com/TekkaGB/Unverum/tree/e41b135e8900187b9a7a06f52bf723f4a7ba35e1)
* **Author:** TekkaGB
* **Repository:** [TekkaGB/Unverum](https://github.com/TekkaGB/Unverum)
* **Role in Project:** The mod manager end users are expected to install our containers with
* **Agent Skill:** `.agents/skills/unverum-mod-packager/` (not yet written; Phase 4)

> [!NOTE]
> The upstream README does not list Sparking! ZERO, but the source does support it (see below).

---

## Reference Documents in this Folder

| File | What it is | Trust level |
|---|---|---|
| [`raw-readme.md`](raw-readme.md) | Upstream README @ `e41b135`, verbatim (outdated game list) | Primary |
| [`source-receipts-setup.md`](source-receipts-setup.md) | Verbatim source: game setup, mods folder, bundled bypass install | Primary |
| [`source-receipts-build.md`](source-receipts-build.md) | Verbatim source: Build button, `Restart` wipe, `CopyFolder`, UE4SS install | Primary |
| [`bundled-ue4ss-settings-ini.md`](bundled-ue4ss-settings-ini.md) | `UE4SS-settings.ini` that ships inside Unverum, verbatim | Primary |

All primary files are script-generated from a clone at `e41b135`. Do not hand-edit them.

---

## Verified Facts for Sparking! ZERO

Setup ([`source-receipts-setup.md`](source-receipts-setup.md)):
- `[FACT]` Expects `SparkingZERO.exe`, project folder `SparkingZERO`, Steam app id `1790600` (`MainWindow.xaml.cs` L395).
- `[FACT]` Mods folder is `<folder of SparkingZERO.exe>\SparkingZERO\Content\Paks\~mods` (`Setup.cs` L139).
- `[FACT]` If `SparkingZERO\Binaries\Win64\plugins\DBSparkingZeroUTOCBypass.asi` is missing, Unverum writes its
  embedded `dsound.dll` and `plugins\DBSparkingZeroUTOCBypass.asi` there (`Setup.cs` L60-L62, L76-L77, L82-L97; `Unverum.csproj` L53-L54).

Build ([`source-receipts-build.md`](source-receipts-build.md)):
- `[FACT]` The Build button runs `CheckPatch`, then `Restart`, then `Build` (`MainWindow.xaml.cs` L826, L827, L844).
- `[FACT]` `Restart` deletes the whole `~mods` folder, `SparkingZERO\Mods` (SZModLib), the UE4SS files
  `opengl32.dll`, `patternsleuth_bind.dll`, `ue4ss.dll`, `UE4SS-settings.ini`, `dwmapi.dll`, plus
  `Binaries\Win64\Mods` and `Content\Paks\LogicMods` (`ModLoader.cs` L18-L74).
- `[FACT]` Each enabled mod goes into its own priority subfolder of `~mods`: `a`, `b`, ... `z`, then `~a`, ...
  (`ModLoader.cs` L275-L279, L303-L308).
- `[FACT]` Each enabled `.pak` is copied as `<name>_9_P.pak`. The `.utoc` and `.ucas` with the same base name are
  copied (also renamed `_9_P`) **only if both exist next to the `.pak`** (`ModLoader.cs` L90, L110-L115).
- `[FACT]` A mod folder containing a `.uplugin` is copied to SZModLib's `SparkingZERO\Mods` instead (`ModLoader.cs` L282-L298).
- `[FACT]` A folder ending in `ue4ss` or `LogicMods` triggers installing Unverum's bundled UE4SS and editing `mods.txt`
  (`ModLoader.cs` L311-L331, L455-L483).

---

## Packaging Implications (for Phase 4)

- `[FACT]` A `.pak` is required. Without it, Unverum ignores our `.utoc`/`.ucas`.
- `[FACT]` Unverum appends `_9_P` itself. A file named `CompleteStory_P.pak` becomes `CompleteStory_P_9_P.pak`.
- `[FACT]` Never use `~mods` as working storage: it is deleted on every build.
- `[HYPOTHESIS]` The engine mount order of `_9_P` files across priority folders. Not tested.
- `[HYPOTHESIS]` Expected release zip layout for Unverum's installer. Not yet sourced; do not state one.

---

## Correction Log

**2026-10-05:** These claims from the first draft were **[DISPROVEN]** or unsourced and were removed:
1. "Unverum enforces `_P` naming": Unverum appends `_9_P` itself.
2. The zip layout `SparkingZERO/Content/Paks/~mods/...` was stated without a source. Now a `[HYPOTHESIS]`.
3. `raw-readme.md` had a fabricated Sparking! ZERO line. Replaced with the true upstream file.

---

## Agent Skill Status
* **Status:** Not started. Receipts are ready; skill is Phase 4 of the implementation plan.
