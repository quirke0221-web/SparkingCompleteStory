# Complete Story: Build, Verification & Testing Runbook

> **Status:** AUTHORITATIVE SSOT  
> **Target Version:** Dragon Ball: Sparking! ZERO (Steam Build `24953175`, Unreal Engine `5.1.1`)  
> **Authoritative Build Orchestrator:** `crates/complete-story-cli`  

---

## 1. Prerequisites & Environment Setup

To build and package Complete Story, the following environment and dependencies are required:

### 1.1 Tooling & Pinned Binaries
* **Rust Toolchain:** `rustc 1.98.1` / `cargo 1.98.1`
* **retoc:** v0.1.5 (`tools/retoc/retoc.exe`)
* **UAssetGUI / UAssetAPI:** v1.1.0 (`tools/UAssetGUI.exe`)
* **RE-UE4SS:** v3.0.1 Beta (`4e5461c`)
* **Mapping File:** `tools/Data/Mappings/SparkingZERO.usmap`

### 1.2 Path Resolution
`complete-story-cli` auto-discovers your Steam game directory (`C:\Program Files (x86)\Steam\steamapps\common\DRAGON BALL Sparking! ZERO`) and local tool binaries in `tools/`. You do not need manual configuration files. To override the Steam location, set:
```powershell
$env:SPARKING_ZERO_GAME_ROOT = "D:\CustomPath\DRAGON BALL Sparking! ZERO"
```

### 1.3 AES Key Environment Variable
Set the AES encryption key for your shell session:
```powershell
$env:SPARKING_ZERO_AES_KEY = "0x..." # Your authorized game key
```
*(Never write or commit the AES key into source files or Git).*

---

## 2. End-to-End Build Pipeline

Build the entire mod with the native Rust orchestrator:

```bash
cargo run -p complete-story-cli -- build
```

### 2.1 The 6 Automated Pipeline Stages
1. **Stage 1 (Extract Stock Assets):** Invokes `retoc to-legacy` with targeted filters to extract `DragonAdventureIFData`, `DragonAdventureIFChartData`, and Goku's `DAIF_CharaData_0000_00` into `staging/legacy/`.
2. **Stage 2 (Deserialize to JSON):** Invokes `UAssetGUI tojson` with `VER_UE5_1` and `SparkingZERO.usmap` into `staging/json/`.
3. **Stage 3 (Pure Domain Transformation):** Uses `serde_json::Value` in `src/transform/` to splice route `0000_00` into character and chart registries and clone Goku's data into `DAIF_CharaData_CompleteStory` without PowerShell array-wrapping bugs.
4. **Stage 4 (Recompile to UAsset):** Invokes `UAssetGUI fromjson` to serialize modified assets into `staging/container/`. Copies `scriptobjects.bin`. Performs post-flight existence and file size assertions.
5. **Stage 5 (Pack IoStore Container):** Invokes `retoc to-zen --version UE5_1` to compile `staging/container/` into `staging/zen/CompleteStory_P.{pak,utoc,ucas}`, then automatically executes `retoc verify`.
6. **Stage 6 (Package Release Archive):** Bundles container files and runtime UE4SS mod into `dist/CompleteStory-Release.zip` using `zip 8.6.0`.

### 2.2 Fast Rebuild & Deployment
If stock assets are already extracted in `staging/legacy/`, skip re-extraction:
```bash
cargo run -p complete-story-cli -- build --skip-extract
```

To build and immediately deploy both container and runtime mod to the game in a single command:
```bash
cargo run -p complete-story-cli -- build --skip-extract --deploy
```

---

## 3. Dedicated CLI Subcommands

`complete-story-cli` provides modular subcommands for fine-grained workflow tasks:

### Pack Staged Assets
Packs whatever is currently staged in `staging/container/` into Zen container files:
```bash
cargo run -p complete-story-cli -- pack
```

### Deploy to Game
Copies the built container files to `Content/Paks/~mods/CompleteStory/` and runtime Lua mod to `Binaries/Win64/Mods/CompleteStory/`:
```bash
cargo run -p complete-story-cli -- deploy
```

### Stream Runtime Logs
Tails the live `ue4ss.log` directly from your game directory:
```bash
cargo run -p complete-story-cli -- logs
```

---

## 4. Automated Testing Suite

Before running live builds, run the automated integration tests:

```bash
cargo test -p complete-story-cli
```

Tests include:
* **AST Schema Integrity:** Verifies 13-element chart array, native `[StructData, ObjectData]` pair, zero PowerShell wrapper bugs.
* **Registry Insertion:** Verifies route key `0000_00` at index 13 and package import appending.
* **RawExport Preservation:** Verifies Base64 Raditz start pointers are preserved byte-for-byte.
* **Fail-Hard Zero-Mock Validation:** Validates that broken or missing data fails hard immediately with diagnostic errors.
* **CLI Parser Validation:** Tests all subcommands and flag combinations.

---

## 5. Verification & Testing Protocol

Before running in-game tests:
1. **Back Up Save Data:** Always back up `MainGameSaveData` from `%LOCALAPPDATA%\SparkingZERO\Saved\SaveGames\` to an external directory.
2. **Isolate Mods:** Ensure no conflicting UI or Episode Battle mods are active.
3. **Bounded Testing Gates:**
   * **Gate 1 (Tile Presentation):** Verify all 12 stock characters remain selectable and the 13th "Complete Story" tile appears.
   * **Gate 2 (Interaction):** Confirm tile displays "New Game".
   * **Gate 3 (Launch):** Verify transition into Goku's Raditz opening battle without prompting for DLC.
