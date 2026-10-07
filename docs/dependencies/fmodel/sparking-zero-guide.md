# FModel: Dragon Ball Sparking! ZERO Integration Guide

> **Status:** AUDITED REFERENCE  
> **Target Game:** Dragon Ball: Sparking! ZERO (Unreal Engine 5.1.1)  
> **Core Library:** CUE4Parse  

---

## 1. Directory & Configuration Setup

To integrate FModel into the Sparking! ZERO modding harness:

1. **Binary Staging:**
   * Binary location: `.tools/FModel/FModel.exe`
2. **Game Directory Target:**
   * Path: `C:\Program Files (x86)\Steam\steamapps\common\DRAGON BALL Sparking! ZERO\SparkingZERO\Content\Paks`
3. **UE Version Override:**
   * In FModel Settings, set UE Version: `GAME_UE5_1` (Unreal Engine 5.1).
4. **AES Decryption Key:**
   * Key: Value from environment variable `$env:SPARKING_ZERO_AES_KEY`.
   * Input format: `0x...` hex string.
5. **Mapping File (.usmap):**
   * Path: `.tools/Data/Mappings/SparkingZERO.usmap` (already present in repository).
   * Required for resolving unversioned property tags in shipping `.uasset` binaries.

---

## 2. Core Operational Recipes

### 2.1 Exploring Episode Battle Assets
* **Target Package:** `SparkingZERO/Content/SS/Blueprints/`
* Assets:
  - `DragonAdventureIFData.uasset`: Master Episode Battle character registry, `PtrRecords` carousel list, and `DefaultOpenCharacter`.
  - `DragonAdventureIFChartData.uasset`: Episode Battle storyline flowcharts, branching nodes, and win/loss conditions.
* In FModel, double-clicking any package parses the IoStore chunk, deserializes the UAsset properties using `SparkingZERO.usmap`, and renders the complete JSON property tree with syntax highlighting.

### 2.2 Finding Story Character Data Assets
* **Target Package:** `SparkingZERO/Content/SS/MasterDataAsset/DragonAdventureIF/`
* Subfolders contain character-specific story metadata:
  - `0000_00`: Custom / test saga templates (`DAIF_CharaData_0000_00.uasset`).
  - `0000_40`: Vanilla Goku saga metadata.
  - `0032_00`: Vegeta saga metadata.

---

## 3. Epistemic Verification Rules
* `[FACT]`: FModel parses UE5 IoStore containers natively using CUE4Parse without requiring external command-line unpacking.
* `[FACT]`: Without `.usmap`, properties with missing schema names appear as `Property_0`, `Property_1`. Providing `SparkingZERO.usmap` restores full human-readable variable names (`PtrRecords`, `DefaultOpenCharacter`, `KoratCharacterDataList`).
* `[POLICY]`: Never commit extracted gigabyte dumps of raw assets to the Git repository. Use FModel for live inspection and extract only targeted packages via CLI `retoc to-legacy`.
