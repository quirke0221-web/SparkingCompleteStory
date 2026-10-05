# Research: Sparking! ZERO Modding Ecosystem & Verified Toolchain

**Date Verified:** 2026-10-04  
**Target Game:** *Dragon Ball: Sparking! ZERO* (PC / Steam build `24953175`)  
**Engine:** Unreal Engine 5.1.1 (Zen / IoStore container architecture)  

---

## 1. Engine Architecture & The Signature Gate

*Dragon Ball: Sparking! ZERO* packages its assets as Unreal Engine 5 IoStore containers (`.pak` + `.utoc` + `.ucas`).
* `[FACT]` `retoc to-zen` writes a sibling `.pak` next to the `.utoc`/`.ucas` (see `docs/dependencies/retoc/source-receipts-behavior.md`).
* `[HYPOTHESIS]` The detailed role of each file (what the `.pak` holds; whether the `.utoc` carries signatures). Not sourced. *(Corrected 2026-10-05: an "RSA digital signatures" claim was removed.)*

### UTOC Signature Bypass
* `[FACT]` It is two files, `dsound.dll` and `plugins\DBSparkingZeroUTOCBypass.asi`, installed into `SparkingZERO\Binaries\Win64` by Unverum (Unverum source @ `e41b135`; see `docs/dependencies/utoc-bypass/README.md`).
* `[HYPOTHESIS]` How it works, and that the game rejects modded containers without it. *(Corrected 2026-10-05: the earlier mechanism text and the `SparkingZERO-Win64-Shipping.exe` name had no primary source. The Nexus page `mods/18` returned 403.)*

---

## 2. Primary Toolchain (Live Verified Dependencies)

The following dependencies were verified via live repository inspection and local configuration checks:

### A. IoStore Container Management: `retoc`
* **Repository:** [trumank/retoc](https://github.com/trumank/retoc)
* **Live Verified Capabilities (from `master/README.md`):**
  * `retoc to-legacy`: Converts modern Zen `.utoc/.ucas` containers to legacy `.uasset` and `.uexp` files so they can be inspected and edited.
  * `retoc to-zen`: Converts edited legacy `.uasset/.uexp` files back into valid UE 5.1 `.utoc/.ucas` IoStore containers.
  * `retoc verify`: Compares each chunk's hash against the container's TOC. It does not check directory indexes, dependencies, or in-game loadability. *(Corrected 2026-10-05; see `docs/dependencies/retoc/cli-reference.md` §5.)*
  * `retoc gen-script-objects`: Builds a script-objects global *container* from a `.jmap` reflection dump. It is **not** used by this project's pipeline. The `scriptobjects.bin` used for packing comes from `to-legacy`, and `to-zen` treats it as optional. *(Corrected 2026-10-05.)*
* **Local Repo Usage:** Directly leveraged in `helpers/Build-CompleteStory.ps1` (Stages 1, 5, 6) and `helpers/Common.ps1`.


### B. Asset Deserialization: `UAssetGUI` / `UAssetAPI`
* **Repository:** [atenfyr/UAssetGUI](https://github.com/atenfyr/UAssetGUI)
* `[FACT]` CLI syntax (`tojson`/`fromjson` argument order, `VER_UE5_1`, `.usmap` mapping name) source-audited against upstream @ `4855f8c` (see `docs/dependencies/uassetgui/`).
* `[LOCAL]` Mapping file `SparkingZERO.usmap`, SHA-256 `B7AE00F54BA558EF7793CABA3437B3E82D64D29A6822C34CE73B4A1F91B2D1C5`.
* **Local Repo Usage:** Leveraged in `helpers/Build-CompleteStory.ps1` (Stages 2, 4) and `helpers/Common.ps1`.

### C. Runtime Scripting & Memory Injection: `RE-UE4SS`
* **Repository:** [UE4SS-RE/RE-UE4SS](https://github.com/UE4SS-RE/RE-UE4SS). Installed build: `v3.0.1 Beta`, Git SHA `4e5461c` (`evidence/runtime/v0.7-sanitized.log`).
* `[FACT]` README @ `4e5461c`: "Targeting UE Versions: From 4.12 To 5.3"; Lua scripting system and C++ modding API (`docs/dependencies/ue4ss/raw-readme.md` lines 10, 25).
* `[FACT]` Native function hooks via `RegisterHook` (`docs/dependencies/ue4ss/upstream-lua-hooks-threading.md`).
* `[POLICY]` Use `bUseUObjectArrayCache = false` (project test ini; Unverum's bundled ini also sets it). Upstream default is `true`.
* `[HYPOTHESIS]` That `true` crashes this game. *(Corrected 2026-10-05: "community guides verify" and the "reallocates its global UObject table" explanation had no source.)*

### D. Mod Managers & Community Distribution
* **Unverum:** [TekkaGB/Unverum](https://github.com/TekkaGB/Unverum)
  * `[FACT]` Source @ `e41b135` supports Sparking! ZERO (its README does not list it). Mods go to `SparkingZERO\Content\Paks\~mods`, each enabled `.pak` copied as `<name>_9_P.pak` with matching `.utoc`/`.ucas`. *(Corrected 2026-10-05: Unverum appends `_9_P` itself; it does not require authors to use `_P`.)* See `docs/dependencies/unverum/README.md`.
* **Reloaded-II:** [Reloaded-Project/Reloaded-II](https://github.com/Reloaded-Project/Reloaded-II)
  * `[HYPOTHESIS]` Community use for Sparking! ZERO audio/video replacement. Not sourced. Not a dependency of this project.
