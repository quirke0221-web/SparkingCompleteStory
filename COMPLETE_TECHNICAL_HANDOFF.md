# Complete Story — Complete Technical Handoff

Status date: 2026-10-04. This is the authoritative engineering handoff. It distinguishes serialized facts, runtime observations, and hypotheses.

## 1. Goal

Complete Story is intended to be a real, separate thirteenth Episode Battle selection. All twelve stock campaigns must remain available. Complete Story should orchestrate existing canonical Z and Super content chronologically across Goku, Piccolo, Gohan, Krillin, Vegeta, and other relevant viewpoints, reusing existing battles, dialogue, cinematics, animation, event configuration, and charts.

It is not a duplicate Goku campaign. The immediate milestone is `Episode Battle → Complete Story → New Game → Goku's original Raditz opening → playable battle`. No build has reached it. Cross-character progression is not implemented.

## 2. Environment

- Steam build `24953175`; Unreal Engine `5.1.1`.
- FModel `4.4.4.0`; retoc `0.1.5`; UAssetGUI/UAssetAPI `1.1.0`; UE4SS `3.0.1 Beta` Git SHA `4e5461c`.
- Mapping SHA-256 `B7AE00F54BA558EF7793CABA3437B3E82D64D29A6822C34CE73B4A1F91B2D1C5`.
- Encrypted IoStore packaging. The owner supplies the AES key through `SPARKING_ZERO_AES_KEY`; it is not stored here.

## 3. Complete architecture

### Character selection

`/Game/SS/Blueprints/DragonAdventureIFData.DragonAdventureIFData` is an `SSDragonAdventureIFDataAsset`. `PtrRecords` is a `TMap<FKoratCharacterDataList, USSDragonAdventureIFCharacterDataAsset*>`; `FKoratCharacterDataList` contains one `FName Key`. Other relevant properties are `DefaultOpenCharacter`, `CameraSequencer`, `BGMDataList`, and `UIAssetArray`.

Observed stock keys, in order:

`0000_40, 0020_60, 0032_00, 0050_00, 0040_00, 0060_00, 0070_00, 0080_30, 0153_00, 0162_00, 0800_00, 0930_00`

The stock default is `0000_40`. v0.3 appends `0000_00 → DAIF_CharaData_CompleteStory`. In-game testing proves a thirteenth tile renders without removing the stock twelve. It does not prove arbitrary keys are valid native campaign identities.

### Chart selection

`/Game/SS/Blueprints/DragonAdventureIFChartData.DragonAdventureIFChartData` is a separate registry. Goku's working selection record is `0000_40 → ChartData0000_00`. v0.3 clones that record under `0000_00`. v0.1 lacked it and soft-locked; v0.3 reached a storefront prompt. Chart registration therefore participates in launch routing but does not initialize playability.

### Character and story data

Original Goku package:
`/Game/SS/MasterDataAsset/DragonAdventureIF/0000_00/DAIF_CharaData_0000_00`

Custom clone:
`/Game/SS/MasterDataAsset/DragonAdventureIF/CompleteStory/DAIF_CharaData_CompleteStory`

Both are `SSDragonAdventureIFCharacterDataAsset`. The custom package/export is renamed and `CharacterName` becomes culture-invariant `Complete Story`; Goku's story graph remains referenced. Goku's asset-path `0000_00` and working selection key `0000_40` belong to different layers and are not interchangeable.

`ChartData0000_00` is an `SSDragonAdventureIFChartCharaDataAsset` connecting character, line, map, result, island/layout, event-block, and story-map UI data. The clone retains:

- `StartEvent = Event_00_0_00_00`
- `StartEventBlock = EventBlock_0000_00`
- `EventData = DIF_Event_0000_00`

Inspected opening blocks include `EventBlock_0000_40_0000_00` and `_01`. Event blocks hold ordered events, chart links, text, next-event data, and optional unlock behavior. `DIF_Event_0000_00` records select battle/story/time-slice/flyer payloads and result-dependent transition, prerequisite, unlock, retry, and difficulty rules. These references are verified; the native travel/load call was not decompiled.

### Save and unlock state

The object dump verifies:

- `FSSDragonAdventureIFSaveData::CharacterPlayableData` is a map keyed by `FKoratCharacterDataList`.
- Values are `FSSDragonAdventureIFCharacterPlayableSaveData`.
- Its `UnlockInfo` uses `EKoratUnLockMode`: `Lock=0`, `New=1`, `Checked=2`, `CheckedLock=3`.
- `SSDragonAdventureIFCSManager::IsPlayable` is parameterless and returns `bool`.
- The manager also exposes `IsModeStart`, list movement, focus, route-clear functions, and `BuiltInMenu`.

The asset registries do not create a `CharacterPlayableData` record. The observed `Unlock` is consistent with that omission, but the native body was not decompiled, so the precise lookup is not proven.

### DLC investigation

`DownLoadContentsData` and records such as `DLC_013` expose `AdventureIFCharacterIds` and `DlcUnlockIgnoreCharacterIds`. Stock DLC 013 values observed were:

- `AdventureIFCharacterIds`: `0050_00, 0060_00, 0070_00, 0162_00`
- `DlcUnlockIgnoreCharacterIds`: `0310_00, 0020_50`

v0.5 appended `0000_00` to `AdventureIFCharacterIds`; the same Unlock label and NEO prompt remained. That edit is disproven. The storefront text is not proof of a real NEO entitlement requirement; an invalid/locked identity may reach an unrelated fallback.

### UI and native managers

Relevant components are `WBP_GRP_AI_CharacterSelect`, `WBP_GRP_AI_Map_0000_40`, `SSMenuManager`, `SSBuiltInMenu`, `SSDragonAdventureIFCSManager`, and `SSDragonAdventureIFCharacterSelectController`.

The character widget has six recycled panel objects. `SSBuiltInMenu` has `EntryItems`, `OnDecided`, `DecideButton`, and `NewDecideButton`. `SSMenuManager` holds navigation/focus state. Presentation and native campaign state are separate.

v0.7 read panel `Index/ShowNum` values `-3/6,-2/4,-1/2,1/2,2/4,3/6`. They are relative carousel positions, not registry index 12. No cooked Blueprint graph has yet been successfully changed to route a thirteenth visual choice to a standalone submenu.

## 4. Complete attempt history

### v0.1 — character registry only

Changed `DragonAdventureIFData` and added `DAIF_CharaData_CompleteStory`. Expected Raditz. Actual result: the separate tile appeared, stock entries remained, and it said `Unlock`. Confirm soft-locked navigation and Back. The two-package IoStore passed parse, round-trip, and retoc verification. Lesson: character registration is insufficient. Exact delta: `history/v0.1/asset-delta.json`.

### v0.2 — IgnoreOpen

Set `IgnoreOpen=true` on the clone, expecting an unlock bypass. Complete Story disappeared when tested by itself. Serialization passed. Do not repeat. Exact delta: `history/v0.2/asset-delta.json`.

### v0.3 — chart registry

Removed the v0.2 change and appended `0000_00 → ChartData0000_00` to the chart registry while keeping `DefaultOpenCharacter=0000_40`. All original entries plus Complete Story appeared. Confirm displayed the NEO storefront; action remained Unlock. The three packages passed decoding, reparse, dependency, and IoStore checks. This is the clean asset baseline, not a playable release. Exact source: `scripts/build_complete_story_assets.ps1`; delta: `history/v0.3/asset-delta.json`.

### v0.4 — default identity

Changed `DefaultOpenCharacter` from `0000_40` to `0000_00`, expecting initialization. Other characters disappeared and selector initialization regressed. Comparison isolated this as the significant registry change. The default is startup/menu state, not an unlock initializer. Exact delta: `history/v0.4/asset-delta.json`.

### v0.5 — DLC registry

Appended `0000_00` to DLC 013 `AdventureIFCharacterIds`, expecting entitlement alignment. Unlock and the identical NEO popup remained. This did not establish a true DLC dependency. Exact delta: `history/v0.5/asset-delta.json`.

### v0.6 — unsafe runtime selection detection

Actual source: `archive/legacy/v0.6-runtime/Scripts/main.lua`. It hooked `IsPlayable`, `DecideButton`, and `NewDecideButton`, then tried to recognize Complete Story through `FindAllOf("TextBlock")`, `FindAllOf("WBP_GRP_AI_CharacterSelect_C")`, `StaticFindObject`, `GetFullName`, UObject properties, and FText conversion.

The initial install did not activate Lua. After correction, the log proved three hooks registered. It did not prove a completed playability override; one diagnostic reported the manager unavailable. Episode Battle crashed. Lua `pcall` does not guarantee protection from native access violations on transient UObjects. The exact crashing operation was not established, so `IsPlayable` itself is not conclusively the cause.

### v0.7 — delayed hook and panel indexes

Actual source: `runtime/v0.7/CompleteStory/Scripts/main.lua`. It removed optional hooks, delayed `IsPlayable` installation 60 seconds, and installed on the game thread. Startup remained stable. At Episode Battle, the callback entered and logged the six panel values above, then the game crashed. No override-applied line followed.

The retained log proves initialization at `00:02:59`, hook installation at `00:04:00`, and callback entry/panel readings at `00:12:23`. It does not identify a native exception address. Lesson: panel reflection and the target-index assumption are invalid.

## 5. Current implementation

`scripts/build_complete_story_assets.ps1` is the complete v0.3 transform. It verifies a twelve-record baseline, `DefaultOpenCharacter=0000_40`, and Raditz pointers; clones/renames the character package; adds imports/dependencies; appends the character record; and appends the chart record reusing Goku's chart import.

Supporting scripts cover targeted extraction, JSON export/import, structural tests, IoStore build/verification, safe install, backed-up uninstall, log sanitization, and audit generation. Exact v0.1–v0.5 authored deltas are under `history/`; actual v0.6/v0.7 Lua is preserved.

Current runtime behavior remains: visible thirteenth selection, Unlock, NEO fallback, no Raditz launch.

## 6. Next technical directions

### A. Transient playable-state initialization

Find where `CharacterPlayableData` is populated. At a stable post-save lifecycle point, test whether `FKoratCharacterDataList{Key=0000_00}` can receive a non-persistent value cloned from Goku or constructed with `UnlockInfo=New`. Resolve the owner once, verify map/value layout, scope to one key, avoid save serialization, and roll back on unload.

Unknowns: caching time, save dirtying, additional mandatory character/event state, and whether launch code accepts the custom identity after playability succeeds. This is not proven safe.

### B. Stable active-selection identity

Trace the selected `FKoratCharacterDataList` from controller/manager state, a selected `SSBuiltInMenuItem`, or the Confirm/`OnDecided` boundary. Because `IsPlayable` has no parameters, the identity must be cached elsewhere. Capture stable primitive/name data outside rendering callbacks; do not scan all UObjects or read recycled panels. `OnDecided` exposes `InIndex` and `InItem`, but mapping those to registry keys is not yet proven.

### C. Standalone menu mode

The final architecture may require a cooked Blueprint override that gives a thirteenth visual action its own submenu rather than invoking character playability. First prove editable cooked graph bytecode and identify the dispatcher/widget switcher. Packaged-only feasibility remains unknown.

## 7. Eventual chronological design

A future orchestrator should maintain its own ordered chapter table but launch each original event in the stock character context it expects. The intended start is Goku's Raditz opening, canonical Goku/Piccolo battle and resolution, then appropriate Piccolo/Gohan training and Saiyan-invasion viewpoints, continuing through Z and Super.

Do not order content by filenames. Inspect prerequisites, result links, chart blocks, playable character, battle payload, and progression effects. Exclude alternate/Sparking branches. Automatic cross-character continuation may be rejected by native transition logic; a chapter-select foundation may be necessary.

## 8. Complete reproduction

1. Clone the repository and install the exact tool versions above.
2. Copy `config/project.local.example.psd1` to ignored `config/project.local.psd1`.
3. Set `SPARKING_ZERO_AES_KEY` only in the current shell.
4. Run `scripts/Extract-RequiredAssets.ps1` against a legitimate installation. It uses narrow retoc UE5.1 filters and redacts arguments on failure.
5. Run `scripts/Export-AssetJson.ps1` with mapping name `SparkingZERO`.
6. Run `scripts/build_complete_story_assets.ps1` with registry, Goku, and chart JSON inputs.
7. Run `scripts/Test-CompleteStoryJson.ps1`.
8. Run `scripts/Import-ModifiedAssets.ps1`; retain local `scriptobjects.bin` at staging root.
9. Run `scripts/Build-IoStore.ps1` and `scripts/Verify-IoStore.ps1`.
10. Decode and re-export in full stock-container context; verify thirteen unique records, imports, default `0000_40`, and Raditz pointers.
11. Use `Install-DevelopmentBuild.ps1 -WhatIf` before installation and `docs/TESTING.md` for a bounded test.
12. Use `Uninstall-DevelopmentBuild.ps1 -WhatIf` before removal. The real action now creates and verifies a restorable backup first.

A clone cannot build without excluded stock assets, mapping, AES key, `scriptobjects.bin`, and third-party tools.

## 9. Remaining unknowns

- Native `IsPlayable` body and selected-state dependency.
- Function routing a locked custom identity to NEO storefront.
- Exact `CharacterPlayableData` initialization/cache lifecycle.
- Whether transient initialization can avoid save writes.
- Additional state required after playability succeeds.
- Whether `0000_00` is reserved beyond Goku's asset-path use.
- Whether the stock chart can progress safely under the custom identity.
- Cooked Blueprint feasibility for a standalone submenu.
- Canonical cross-character event list and transition requirements.
- Achievement/route-clear behavior with dynamic registry length.
- Exact v0.6/v0.7 native crash site; no useful minidump or CrashContext was recovered.

## 10. Evidence and provenance

- Source/deltas: `history/README.md`, `archive/legacy/`, `runtime/v0.7/`.
- Logs: `evidence/runtime/v0.7-full-sanitized.log` and concise excerpt.
- Object symbols: `evidence/object-dump/targeted-symbols.txt`.
- Asset evidence: `history/v0.1`–`v0.5`, `evidence/assets/`.
- Package hashes: `evidence/packages/SHA256SUMS.txt`.
- Restricted material: `local-handoff/complete-project-materials-2026-10-04/README.md`.

No statement here implies playability. The highest verified result is the mounted v0.3 tile and its observed locked/storefront behavior.
