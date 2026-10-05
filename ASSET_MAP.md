# Complete Story — Episode Battle Asset Map

Updated: 2026-10-02

Evidence sources are the build-24953175 IoStore index and targeted FModel views,
the retained registry export, and the same build's UE4SS runtime object dump.
Paths below are verified package/object paths unless explicitly marked pending.

Packaging note: the retained FModel blob was not edited. The final prototype
was converted from the encrypted stock IoStore with package-store metadata,
edited as mapped legacy assets, and repacked as a verified UE5.1 IoStore
overlay. Its new package path is
`/Game/SS/MasterDataAsset/DragonAdventureIF/CompleteStory/DAIF_CharaData_CompleteStory`.

## Registration root

### `/Game/SS/Blueprints/DragonAdventureIFData.DragonAdventureIFData`

Class: `SSDragonAdventureIFDataAsset`

Relevant properties:

| Property | Reflected type | Purpose |
|---|---|---|
| `PtrRecords` | `TMap<FKoratCharacterDataList, USSDragonAdventureIFCharacterDataAsset*>` | Selectable campaign registry |
| `DefaultOpenCharacter` | `FKoratCharacterDataList` | Default available campaign |
| `CameraSequencer` | soft Level Sequence pointer | Selection presentation |
| `BGMDataList` | `FKoratBGMDataList` | Selection scene music |
| `UIAssetArray` | name-to-soft-class map | Selection UI classes |

The retained cooked asset contains references to:

- `/Game/SS/Scene/DragonAdventureIF/CharacterSelect/MS_menuDAIF_CS`
- `/Game/SS/UI/AdventureIF/WBP_GRP_AI_CharacterSelect`
- `/Game/SS/UI/AdventureIF/WBP_GRP_AI_CharacterSelect_3piece`

`FKoratCharacterDataList` is a one-field struct containing `FName Key`.
The current registry was previously observed with 12 entries. The retired Lua
diagnostic is not part of the packaged implementation.

## Per-campaign route data

### `/Game/SS/MasterDataAsset/DragonAdventureIF/0000_00/DAIF_CharaData_0000_00`

Class: `SSDragonAdventureIFCharacterDataAsset`

This is Goku's campaign definition and the value aliased by the first
thirteenth-entry prototype. Relevant reflected properties include:

- `CharacterName`, `IntroductionText`, `CostumeDataList`
- selection actor/locator/action/camera presentation
- `IgnoreOpen`, `IsShowSynopsis`
- `StartData`
- `EventData`
- `StartEvent`
- `StartEventBlock`
- `RouteClearInfoArray`
- update/DLC and directing settings

Verified Goku entry values from the targeted FModel inspection:

- `StartEvent`: `Event_00_0_00_00`
- `StartEventBlock`: `EventBlock_0000_00`
- `EventData`: `DIF_Event_0000_00`

A separately displayed Complete Story entry needs its own object of this class.
Aliasing this object is suitable only for the capacity/launch experiment because
editing its display fields would also affect the original Goku entry.

## Goku route graph

### `/Game/SS/MasterDataAsset/DragonAdventureIF/0000_00/ChartData0000_00`

Class: `SSDragonAdventureIFChartCharaDataAsset`

Connects the character data to line data, map data, episode-result data, island
layout, an event-block map, and story-map UI classes.

### Opening event blocks inspected

- `/Game/SS/MasterDataAsset/DragonAdventureIF/0000_00/ChartData/EventBlock/EventBlock_0000_40_0000_00`
- `/Game/SS/MasterDataAsset/DragonAdventureIF/0000_00/ChartData/EventBlock/EventBlock_0000_40_0000_01`

Event-block assets provide the chart title/explanation/route text, ordered event
names, directional links, next-event information, and optional unlock behavior.

### `/Game/SS/MasterDataAsset/DragonAdventureIF/0000_00/EventData/DIF_Event_0000_00`

Class: `SSDragonAdventureIFEventDataAsset`

Its event records select battle/story/time-slice/flyer payloads and contain
result-based next-event, branch, prerequisite, unlock, retry, and difficulty
rules. This is the progression layer that will eventually join canonical events
from multiple character stories.

## Selection UI and controller

### `/Game/SS/UI/AdventureIF/WBP_GRP_AI_CharacterSelect`

The widget has six reusable panel objects, numbered `_0` through `_5`, plus
loop/array logic and up/down list events. It does not contain twelve separately
reflected campaign panels. This suggests reuse/pagination, but support for a
thirteenth record is not proven.

Relevant native classes/functions:

- `SSDragonAdventureIFCharacterSelectController`
  - `CharacterManager`
  - `SelectMenu`
- `SSDragonAdventureIFCSManager`
  - `IsPlayable`
  - `IsModeStart`
- `SSDragonAdventureIFEventDataAsset::DataTransition`

### `/Game/SS/Blueprints/DragonAdventureIF/BP_DragonAdventureIFChartSelecterByPad`

Targeted FModel inspection confirmed this chart-selection Blueprint path. It is
relevant after a campaign is launched, not the primary registry for the first
menu entry.

## Save/progression structures

These are reflected native structures rather than standalone package paths:

- `FSSDragonAdventureIFSaveData`
  - `CharacterData`: character-keyed map
  - `CharacterPlayableData`: character-keyed map
  - `bDifficultyEasy`
- `FSSDragonAdventureIFCharacterSaveData`
  - `EventBlockDataMap`
  - `EventDataMap`
  - `LastEventBlockName`
  - unlock/activity data
- `FSSDragonAdventureIFEventSaveData`
  - result, unlock, clear-count, reward state

An added key fits the serialized container types, but initialization and total/
trophy behavior remain unverified. The first prototype test must stop at the
selector.

## Answers

### Where is the selectable-character registry?

`/Game/SS/Blueprints/DragonAdventureIFData.DragonAdventureIFData`, property
`PtrRecords`.

### Can routes be registered through data assets?

Yes at the data-model level: each map value is a route/presentation data asset
that points to its starting block/event and event-data graph. Runtime acceptance
of the thirteenth entry is pending the v0.4 test.

### Are menu entries/counts dynamic or hardcoded?

The registry and saves use dynamic maps. The widget reuses six panels and has
loop logic. No hardcoded 12 was found in reflected fields, but compiled widget
navigation for a thirteenth record is not proven.

### What controls Goku's initial story event?

`DAIF_CharaData_0000_00` through `StartEvent`, `StartEventBlock`, and
`EventData`, with the verified values listed above.

### Could a route affect saves or existing progression?

Yes. A new character key creates a new potential save namespace and may interact
with playability, unlock initialization, route-clear totals, rewards, and
trophies. The format is map-based, but safe behavior needs a controlled test.

## Minimal next asset set

After the registry-capacity test passes, create/examine only:

1. One new `CompleteStory` character data asset based on Goku's verified asset.
2. The smallest registry extension that points a unique, validated key to it.
3. Name/introduction/portrait presentation dependencies for the new selection.
4. Save/playability initialization for that key.
5. No custom chart/event graph until the separate entry launches Goku's stock
   opening successfully.
