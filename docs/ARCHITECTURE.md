# Episode Battle Architecture

## Verified asset chain

`/Game/SS/Blueprints/DragonAdventureIFData` is an `SSDragonAdventureIFDataAsset`. Its `PtrRecords` map uses `FKoratCharacterDataList` (one `FName Key`) and points to `SSDragonAdventureIFCharacterDataAsset`. The stock registry has twelve records. v0.3 appends `0000_00 → DAIF_CharaData_CompleteStory` and preserves `DefaultOpenCharacter = 0000_40`.

`/Game/SS/Blueprints/DragonAdventureIFChartData` is a separate chart registry. v0.3 clones the working Goku `0000_40` record, changes only the key to `0000_00`, and retains its reference to `ChartData0000_00`. v0.1 lacked this mapping and soft-locked; v0.3 reached a storefront prompt. Chart registration changed the path but did not make the custom identity playable.

Original Goku data is `/Game/SS/MasterDataAsset/DragonAdventureIF/0000_00/DAIF_CharaData_0000_00`. The custom clone is `/Game/SS/MasterDataAsset/DragonAdventureIF/CompleteStory/DAIF_CharaData_CompleteStory`. The source package suffix `0000_00` and selection key `0000_40` are not interchangeable identifiers.

The clone retains `StartEvent = Event_00_0_00_00`, `StartEventBlock = EventBlock_0000_00`, and `EventData = DIF_Event_0000_00`. `ChartData0000_00` connects character, line, map, result, event-block, island, and map-UI data. Event records select battle/story payloads and result-dependent transitions. The references are verified; all native launch prerequisites are not.

## Playability and save state

The object dump verifies that `FSSDragonAdventureIFSaveData::CharacterPlayableData` is a map keyed by `FKoratCharacterDataList` with `FSSDragonAdventureIFCharacterPlayableSaveData` values. The value has `UnlockInfo : EKoratUnLockMode`; enum values are `Lock`, `New`, `Checked`, and `CheckedLock`. `SSDragonAdventureIFCSManager::IsPlayable` is parameterless and returns `bool`.

Adding registry records does not create a corresponding playable-state record. Runtime behavior is consistent with that omission causing `Unlock`, but the native function body was not decompiled. The exact NEO storefront fallback path remains an inference.

v0.5 added `0000_00` to DLC 013 `AdventureIFCharacterIds`; runtime behavior did not change. That disproved the attempted fix, not the existence or absence of a broader entitlement path.

## UI/native boundary

`WBP_GRP_AI_CharacterSelect` has six recycled panel objects. `SSMenuManager` owns navigation state. `SSBuiltInMenu` exposes `EntryItems`, `DecideButton`, `NewDecideButton`, and `OnDecided`. `SSDragonAdventureIFCSManager` owns a `BuiltInMenu` reference and exposes `IsPlayable`, `IsModeStart`, movement, focus, and route-clear functions.

v0.7 logged panel `Index/ShowNum` values `-3/6,-2/4,-1/2,1/2,2/4,3/6`. They are carousel positions, not the thirteenth registry index. Reflecting those panels inside `IsPlayable` was invalid and crashed.

Confirmed: additive registry serialization, thirteenth tile rendering, chart registration, Raditz references, and reflected save/function types. Unresolved: stable selected-key ownership, native storefront routing, safe playable-state creation, standalone submenu dispatch, and cross-character continuation.
