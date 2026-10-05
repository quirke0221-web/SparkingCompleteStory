# Dragon Ball Sparking! ZERO: Complete Story Asset Data Dictionary

> **Status:** AUTHORITATIVE REFERENCE  
> **Source Evidence:** Extracted from Steam build `24953175`, Unreal Engine `5.1.1` IoStore indexes, FModel `4.4.4.0` exports, and RE-UE4SS `3.0.1 Beta` runtime object dumps.

---

## 1. Selectable Campaign Master Registry

### Package: `/Game/SS/Blueprints/DragonAdventureIFData.DragonAdventureIFData`
* **Class:** `SSDragonAdventureIFDataAsset` (inherits from `UDataAsset`)
* **Primary Properties:**

| Property | Native Offset | Reflected Type | Description / Mod Invariants |
| :--- | :--- | :--- | :--- |
| `PtrRecords` | `0x40` | `TMap<FKoratCharacterDataList, USSDragonAdventureIFCharacterDataAsset*>` | Selectable campaign registry mapping character keys to campaign data assets. |
| `DefaultOpenCharacter` | — | `FKoratCharacterDataList` | Default campaign open on startup. **Must strictly remain `0000_40`** (Goku). Mutating breaks menu init. |
| `CameraSequencer` | — | Soft Object Pointer (`LevelSequence`) | Presentation camera sequencer for character select. |
| `BGMDataList` | — | `FKoratBGMDataList` | Background music played during Episode Battle selection. |
| `UIAssetArray` | — | `TMap<FName, TSoftClassPtr<UUserWidget>>` | Widget classes for selection UI. |

* **Key Struct: `FKoratCharacterDataList`:**
  * Single-field script struct containing `FName Key` at native offset `0`.
* **Stock Vanilla Key Order (12 Campaigns):**
  1. `0000_40` (Goku)
  2. `0020_60` (Vegeta)
  3. `0032_00` (Gohan)
  4. `0050_00` (Piccolo)
  5. `0040_00` (Future Trunks)
  6. `0060_00` (Frieza)
  7. `0070_00` (Goku Black)
  8. `0080_30` (Jiren)
  9. `0153_00` (DLC / Stock Slot 9)
  10. `0162_00` (DLC / Stock Slot 10)
  11. `0800_00` (Stock Slot 11)
  12. `0930_00` (Stock Slot 12)
* **Complete Story Route Key:**
  * Appends `0000_00` as entry 13, pointing to `DAIF_CharaData_CompleteStory`.

---

## 2. Chart Master Registry

### Package: `/Game/SS/Blueprints/DragonAdventureIFChartData.DragonAdventureIFChartData`
* **Class:** `SSDragonAdventureIFChartDataAsset`
* **Dual-Registration Invariant:**
  * In addition to `DragonAdventureIFData`, Episode Battle requires a corresponding entry in `DragonAdventureIFChartData`.
  * Vanilla Goku Record: `0000_40 -> ChartData0000_00`.
  * Complete Story Record: Appends `0000_00 -> ChartData0000_00`. Reusing Goku's stock chart prevents navigation soft-locks and routes the confirmation prompt to the launch handler.

---

## 3. Campaign & Story Data Assets

### Source Goku Package: `/Game/SS/MasterDataAsset/DragonAdventureIF/0000_00/DAIF_CharaData_0000_00`
### Custom Clone: `/Game/SS/MasterDataAsset/DragonAdventureIF/CompleteStory/DAIF_CharaData_CompleteStory`
* **Class:** `SSDragonAdventureIFCharacterDataAsset`
* **Verified Starting Event Pointers:**

| Property | Reflected Value | Description |
| :--- | :--- | :--- |
| `StartEvent` | `Event_00_0_00_00` | Canonical opening dialogue/cutscene event. |
| `StartEventBlock` | `EventBlock_0000_00` | Opening event block in chart graph. |
| `EventData` | `DIF_Event_0000_00` (Index `-7`) | Event payload containing Raditz battle specification. |
| `CharacterName` | CultureInvariant: `"Complete Story"` | Display name text (`Flags = 2`, `HistoryType = None`). |

* **Chart Data Asset: `/Game/SS/MasterDataAsset/DragonAdventureIF/0000_00/ChartData0000_00`:**
  * Class: `SSDragonAdventureIFChartCharaDataAsset`
  * Connects character data to line data, map data, episode-result data, island layout, event-block map, and story-map UI classes.
* **Opening Event Blocks:**
  * `/Game/SS/MasterDataAsset/DragonAdventureIF/0000_00/ChartData/EventBlock/EventBlock_0000_40_0000_00`
  * `/Game/SS/MasterDataAsset/DragonAdventureIF/0000_00/ChartData/EventBlock/EventBlock_0000_40_0000_01`
  * Event blocks define chart title/explanation, directional transitions, next-event links, and unlock requirements.

---

## 4. Native Save Data & Playability Structures

* **Save Game Layout (`MainGameSaveData`):**
  * `FSSDragonAdventureIFSaveData::CharacterPlayableData`: `TMap<FKoratCharacterDataList, FSSDragonAdventureIFCharacterPlayableSaveData>`
  * `UnlockInfo`: Enum `EKoratUnLockMode`
    * `Lock = 0` (Locked)
    * `New = 1` (Newly Unlocked)
    * `Checked = 2` (Viewed by player)
    * `CheckedLock = 3` (Explicitly re-locked)
* **Native Playability Function:**
  * Class: `SSDragonAdventureIFCSManager`
  * Method: `IsPlayable()` (Parameterless native function returning `bool`).
  * **Mechanism:** When selecting a campaign tile, the engine calls `IsPlayable()`. If `CharacterPlayableData` does not contain the key, it defaults to locked (`Unlock` label) and confirm triggers the storefront fallback.
  * **Runtime Solution:** Intercept `IsPlayable()` on GameThread via RE-UE4SS to return `true` when route `0000_00` is queried, avoiding any permanent writes to vanilla save data.

---

## 5. Selection UI & Widget Hierarchy

* **Character Select Widget:** `/Game/SS/UI/AdventureIF/WBP_GRP_AI_CharacterSelect`
  * Contains 6 reusable panel objects: `WBP_OBJ_AI_CharacterPanel_0` through `_5`.
  * **Carousel Mechanics:** Panels represent relative circular offsets (`-3/6, -2/4, -1/2, 1/2, 2/4, 3/6`) around the active center, not absolute registry indices.
  * **Memory Hazard:** Slate widgets are transient and pooled by Unreal Engine. Never reflect, scrape, or store widget pointers across frames (ADR 0004).
