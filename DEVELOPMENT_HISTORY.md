# Complete Story development history

Target: Steam build `24953175`, Unreal Engine 5.1.1, packaged IoStore content.

## Asset architecture established

- Character selector registry: `/Game/SS/Blueprints/DragonAdventureIFData`
- Chart registry: `/Game/SS/Blueprints/DragonAdventureIFChartData`
- Custom key used by the prototypes: `0000_00`
- Original Goku selector key and default: `0000_40`
- Custom character data: `/Game/SS/MasterDataAsset/DragonAdventureIF/CompleteStory/DAIF_CharaData_CompleteStory`
- Reused Goku chart: `ChartData0000_00`
- Raditz start: `Event_00_0_00_00`, `EventBlock_0000_00`, and `DIF_Event_0000_00`

## Runtime test history

### v0.1

- Added Complete Story as a visible 13th Episode Battle selection.
- Preserved the original 12 selections.
- Displayed `Unlock` instead of `New Game`.
- Confirming the selection soft-locked the character menu.

### v0.2

- Added `IgnoreOpen=true`.
- Complete Story disappeared from the selector.
- Rejected as a regression.

### v0.3

- Restored the v0.1 character registry and added the missing chart-registry entry.
- Complete Story remained visible with the original campaigns intact.
- Confirming it opened the unrelated NEO storefront popup instead of the chart.
- This is the clean cooked-asset baseline for later experiments.

### v0.4

- Changed `DefaultOpenCharacter` from `0000_40` to `0000_00`.
- Broke selector initialization and caused original entries to disappear.
- Rejected; the stock default must remain `0000_40`.

### v0.5

- Experimented with DLC-related records.
- The `Unlock` state and NEO storefront popup remained.
- Rejected; legitimate DLC behavior must remain untouched.

### v0.6

- Added UE4SS hooks for `IsPlayable`, `DecideButton`, and `NewDecideButton`.
- Used unsafe global UObject/text-widget searches and reflected asset lookups inside callbacks.
- Crashed or exited around Episode Battle initialization without proving an override.
- Preserved under `archive/legacy/v0.6-runtime` only as a failure reference.

### v0.7

- Removed the optional menu-button hooks and global object scans.
- Immediate `IsPlayable` hook registration still caused startup exit code `3` before the callback.
- Delaying hook installation by 60 seconds and installing on the game thread prevented that startup exit.
- Entering Episode Battle invoked the callback and logged panel values:

  `-3/6,-2/4,-1/2,1/2,2/4,3/6`

- The game then crashed. Those values are relative carousel positions surrounding the selected center, not registry indices. The attempted comparison with registry index `12` is invalid.
- v0.7 is a failing diagnostic reproduction, not a playable release.

## Current blocker

The custom registry key has no native `CharacterPlayableData` state. A safe runtime solution must obtain the actual active campaign identity from stable manager/native state without scanning widgets or dereferencing transient panel objects inside `IsPlayable`. No working implementation has yet demonstrated `Complete Story -> New Game -> Raditz`.
