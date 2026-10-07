# Episode Battle Subsystem: Asset Hierarchy & Schemas

> **Status:** AUDITED RESEARCH RECORD  
> **Target Subsystem:** `DragonAdventureIF` Asset Pipeline  
> **Source Receipts:**  
> - `build/staging/json/DragonAdventureIFData.json` (lines 20–710)  
> - `build/staging/json/DAIF_CharaData_0000_00.json` (lines 1–349)  
> - `evidence/object-dump/targeted-symbols.txt` (commit `e719f6b0`, lines 46361–72923)  
> - `evidence/assets/structural-summary.md` (commit `e719f6b0`)  

---

## 1. Asset Tree Architecture

The Episode Battle data layer consists of three interdependent Unreal Engine 5.1 DataAssets:

```
SparkingZERO/Content/SS/
├── Blueprints/
│   ├── DragonAdventureIFData.uasset       <-- Master Character Carousel & Unlock Registry
│   └── DragonAdventureIFChartData.uasset  <-- Global Mission Flowchart & Node Tree
└── MasterDataAsset/DragonAdventureIF/
    ├── 0000_40/DAIF_CharaData_0000_40.uasset  <-- Vanilla Goku Saga Metadata
    ├── 0032_00/DAIF_CharaData_0032_00.uasset  <-- Vanilla Vegeta Saga Metadata
    └── 0000_00/DAIF_CharaData_0000_00.uasset  <-- Complete Story Saga Metadata
```

---

## 2. Master Registry Schema (`DragonAdventureIFData.uasset`)

The master registry defines the character carousel displayed on the Episode Battle selection ring.

### 2.1 The `PtrRecords` Array
* `[FACT]`: `PtrRecords` contains an ordered array of character entries that populate the 3D menu carousel.
* `[FACT]`: In the vanilla game, exactly 12 records exist:
  ```text
  0000_40 (Goku)
  0020_60 (Vegeta)
  0032_00 (Gohan)
  0050_00 (Piccolo)
  0040_00 (Future Trunks)
  0060_00 (Frieza)
  0070_00 (Goku Black)
  0080_30 (Jiren)
  0153_00 (Vanilla Saga 9)
  0162_00 (Vanilla Saga 10)
  0800_00 (Vanilla Saga 11)
  0930_00 (Vanilla Saga 12)
  ```
* `[FACT]`: Each entry in `PtrRecords` is a 2-element tuple:
  1. `Key` (`NamePropertyData`): The `FKoratCharacterDataList` key (e.g. `"0000_40"`, `"0000_00"`).
  2. `DataAsset` (`ObjectPropertyData`): Negative import index pointing to the character's `SSDragonAdventureIFCharacterDataAsset`.

### 2.2 The `DefaultOpenCharacter` Property
* `[FACT]`: Defined at line 682 of `DragonAdventureIFData.json`:
  ```json
  {
    "$type": "UAssetAPI.PropertyTypes.Structs.StructPropertyData, UAssetAPI",
    "StructType": "KoratCharacterDataList",
    "Name": "DefaultOpenCharacter",
    "Value": [
      {
        "$type": "UAssetAPI.PropertyTypes.Objects.NamePropertyData, UAssetAPI",
        "Name": "Key",
        "Value": "0000_40"
      }
    ]
  }
  ```
* `[FACT]`: In the vanilla shipping game, `DefaultOpenCharacter` is set to `"0000_40"` (Goku). This grants Goku immediate unlocked status on a brand-new save file where `CharacterPlayableData` is empty.

---

## 3. Character Story Metadata Schema (`DAIF_CharaData_*.uasset`)

Each character campaign references a dedicated `SSDragonAdventureIFCharacterDataAsset` defining UI presentation and narrative entry points.

### 3.1 String Table Localization References
* `[FACT]`: Character names and descriptions are decoupled from the binary asset into Unreal Engine String Tables:
  - **Name String Table:** `/Game/SS/StringTables/Event/ST_ADIF_CHR_NAME`
    - Key for Complete Story: `ST_ADIF_CHR_NAME_0000_00`
  - **Synopsis String Table:** `/Game/SS/StringTables/Event/ST_ADIF_SYNOPSIS`
    - Key for Complete Story: `ST_ADIF_SYNOPSIS_00_0_99`

### 3.2 Narrative Start Pointers
* `[FACT]`: The character asset links directly to the opening cinematic and flowchart node:
  - `Event_00_0_00_00` (Opening Event)
  - `EventBlock_0000_00` (Initial Event Block)
  - `DIF_Event_0000_00` (`SSDragonAdventureIFEventDataAsset`)
  - `ChartData0000_00` (Starting Flowchart Root)

---

## 4. Differential Matrix: Vanilla Goku (`0000_40`) vs. Complete Story (`0000_00`)

| Dimension / Property | Vanilla Goku (`0000_40`) | Complete Story (`0000_00`) | Engine Impact & Finding |
|---|---|---|---|
| **Carousel Key** | `0000_40` | `0000_00` | Both present in `PtrRecords` (12 stock + 1 custom). |
| **DefaultOpenCharacter** | `"0000_40"` (Stock) | Mutated to `"0000_00"` | `[OBSERVATION]` Setting this alone in the asset does not switch the tile badge. |
| **DLC Entitlement** | None (Base Game) | None (Base Game) | `[FACT]` Registered outside `DownLoadContentsData`. |
| **Save Data Persistence** | Registered in `CharacterPlayableData` | Missing from `CharacterPlayableData` | `[FACT]` On fresh save, save map contains entries evaluated via `EKoratUnLockMode`. |
| **Flowchart Node Root** | Points to stock Goku saga | Re-uses Raditz start pointers | Flowchart pointers are syntactically valid but require runtime traversal verification. |

---

## 5. Epistemic Assessment

* `[FACT]`: The asset layer is syntactically sound; the game mounts `CompleteStory_P.pak/.utoc/.ucas` and renders the 13th carousel slot without crashes.
* `[OBSERVATION]`: The persistent lock state is **NOT an asset serialization error** (IoStore hashes and UAsset headers are valid).
* `[HYPOTHESIS]`: The lock is driven by the evaluation logic in `SSDragonAdventureIFCSManager:IsPlayable` querying `CharacterPlayableData` and falling back to the DLC store when the character key is absent from the save data map.
