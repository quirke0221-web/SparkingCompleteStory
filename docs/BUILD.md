# Complete Story: Build, Verification & Testing Runbook

> **Status:** AUTHORITATIVE SSOT  
> **Target Version:** Dragon Ball: Sparking! ZERO (Steam Build `24953175`, Unreal Engine `5.1.1`)

---

## 1. Prerequisites & Environment Setup

To build and package Complete Story, the following environment and dependencies are required:

### 1.1 Tooling & Pinned Binaries
* **retoc:** v0.1.5 (`portable/retoc.exe`, SHA-256 `005a0da...`)
* **UAssetGUI / UAssetAPI:** v1.1.0 (`UAssetGUI.exe`, SHA-256 `895781a...`)
* **RE-UE4SS:** v3.0.1 Beta (`4e5461c`)
* **Mapping File:** `SparkingZERO.usmap` (2,369,253 bytes, SHA-256 `B7AE00F54BA558EF7793CABA3437B3E82D64D29A6822C34CE73B4A1F91B2D1C5`)

### 1.2 Local Configuration (`config/project.local.psd1`)
Copy `config/project.local.example.psd1` to `config/project.local.psd1` (ignored by Git) and define:
```powershell
@{
    GameRoot                 = 'C:\SteamLibrary\steamapps\common\DRAGON BALL Sparking! ZERO'
    RetocPath                = 'C:\modding\tools\retoc.exe'
    UAssetGUIPath            = 'C:\modding\tools\UAssetGUI.exe'
    MappingName              = 'SparkingZERO'
    MappingPath              = 'C:\modding\mappings\SparkingZERO.usmap'
    AesKeyEnvironmentVariable = 'SPARKING_ZERO_AES_KEY'
}
```

### 1.3 AES Key Environment Variable
Set the AES encryption key for your shell session:
```powershell
$env:SPARKING_ZERO_AES_KEY = "0x..." # Your authorized game key
```
*(Never write or commit the AES key into configuration files or scripts).*

---

## 2. End-to-End Build Pipeline

Build the entire mod with one unified orchestrator cmdlet:

```powershell
.\helpers\Build-CompleteStory.ps1
```

### 2.1 The 6 Automated Pipeline Stages
1. **Stage 1 (Extract Stock Assets):** Invokes `retoc to-legacy` with targeted filters to extract `DragonAdventureIFData`, `DragonAdventureIFChartData`, and Goku's `DAIF_CharaData_0000_00` into `staging/legacy/`.
2. **Stage 2 (Deserialize to JSON):** Invokes `UAssetGUI tojson` with `VER_UE5_1` and `SparkingZERO.usmap` into `staging/json/`.
3. **Stage 3 (Pure Domain Transformation):** Calls `helpers/Transform-CompleteStoryAssets.ps1` to splice route `0000_00` into character and chart registries and clone Goku's data into `DAIF_CharaData_CompleteStory` in `staging/modified-json/`.
4. **Stage 4 (Recompile to UAsset):** Invokes `UAssetGUI fromjson` to serialize modified assets into `staging/container/`. Copies `scriptobjects.bin`.
5. **Stage 5 (Pack IoStore Container):** Invokes `retoc to-zen --version UE5_1` to compile `staging/container/` into `dist/CompleteStory_P.{pak,utoc,ucas}`, then runs `retoc verify`.
6. **Stage 6 (Package for Unverum):** Bundles container files into `dist/CompleteStory-v0.3-Unverum.zip`.

### 2.2 Rebuild Acceleration & One-Step Deployment
If stock assets are already extracted in `staging/legacy/`, skip re-extraction:
```powershell
.\helpers\Build-CompleteStory.ps1 -SkipExtraction
```

To build and immediately deploy both container and runtime mod to the game in a single command:
```powershell
.\helpers\Build-CompleteStory.ps1 -SkipExtraction -Deploy
```

---

## 3. Local Developer Deployment

For testing locally without Unverum, use the developer deployment utility:

### Install Development Build
```powershell
# Backs up any existing installation and copies dist/ container files to ~mods\CompleteStory\
.\helpers\Deploy-DevelopmentBuild.ps1 -Install
```

### Uninstall Development Build
```powershell
# Verifies a restorable backup in local-handoff/deploy-backups/ and removes ~mods\CompleteStory\
.\helpers\Deploy-DevelopmentBuild.ps1 -Uninstall
```

---

## 4. Verification & Testing Protocol

Before running in-game tests:
1. **Back Up Save Data:** Always back up `MainGameSaveData` from `%LOCALAPPDATA%\SparkingZERO\Saved\SaveGames\` to an external directory.
2. **Isolate Mods:** Ensure no conflicting UI or Episode Battle mods are active.
3. **Bounded Testing Gates:**
   * **Gate 1 (Tile Presentation):** Verify all 12 stock characters remain selectable and the 13th "Complete Story" tile appears.
   * **Gate 2 (Interaction):** Confirm tile selection.
   * **Gate 3 (Launch):** Verify transition into Goku's Raditz opening battle without native exceptions.

---

## 5. Automated Crash & Log Diagnostics

If the game crashes, freezes, or encounters an error during testing, inspect logs immediately:
```powershell
# Tails the last 50 lines of ue4ss.log, SparkingZERO.log, and recent crash dumps
.\helpers\Get-ModLogs.ps1

# Inspect strictly error, fatal, and access violation patterns
.\helpers\Get-ModLogs.ps1 -ErrorsOnly
```
