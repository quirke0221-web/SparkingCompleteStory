# Complete Story — Project State

Last updated: 2026-10-02

## Current status

A normal UE5.1 IoStore prototype has been built and verified offline:

`dist/CompleteStory-v0.1/CompleteStory_P.{pak,utoc,ucas}`

An Unverum-ready archive is at:

`dist/CompleteStory-v0.1-Unverum.zip`

The mod has not been installed or run. It is an in-game test candidate, not a
claim that the thirteenth menu item renders or that save initialization works.
The game installation, installed mods, and saves remain unchanged.

## Implemented assets

- Overrides
  `/Game/SS/Blueprints/DragonAdventureIFData.DragonAdventureIFData`.
- Preserves the original 12 `PtrRecords` entries in their original order.
- Appends route key `0000_00` as record 13.
- Adds
  `/Game/SS/MasterDataAsset/DragonAdventureIF/CompleteStory/DAIF_CharaData_CompleteStory`.
- Uses a separate `SSDragonAdventureIFCharacterDataAsset` with the invariant
  display text `Complete Story`.
- Retains Goku's verified canonical starting data:
  `Event_00_0_00_00`, `EventBlock_0000_00`, and `DIF_Event_0000_00`.
- Reuses Goku's presentation, synopsis, route-clear, and event references for
  this smallest prototype.

## Packaging and validation

- Source game: Steam build `24953175`, UE `5.1.1`.
- Targeted Zen-to-legacy conversion succeeded with retoc `0.1.5`.
- Both source assets parsed with the exact build mapping in UAssetAPI 1.1.0.
- Unmodified `.uexp` round trips were byte-identical.
- Retoc/UAssetAPI header differences were confined to reconstructed package
  headers/name hashes; serialized properties remained intact.
- Modified binaries reparsed with 13 records, 29 imports, and no unknown
  imports before packaging.
- Final container contains exactly two packages and passes `retoc verify`.
- The registry store entry contains all 12 original imported package IDs plus
  the new Complete Story package ID.
- The new character package contains the same three imported package IDs as
  Goku's source asset.
- A full-context final decode alongside the untouched base container resolved
  every import name and reproduced both edited exports. Dependency ordering was
  normalized by retoc, but the dependency set was unchanged.

Archive hashes are recorded in
`dist/CompleteStory-v0.1/SHA256SUMS.txt`. The Unverum zip SHA-256 is
`62A6B4924E066AAE46920B9992AFFF6D061945AD1799BD77790510CF3C180506`.

## Remaining runtime gates

1. Confirm the selector renders/navigates a thirteenth item. Prior inspection
   found six reusable panels and dynamic-looking navigation, but no offline test
   can prove the compiled control flow accepts a third page/extra item.
2. Confirm `SSDragonAdventureIFCSManager::IsPlayable` initializes the new key.
3. Confirm selection launches the original Raditz opening.
4. Observe whether save initialization, route-clear totals, or trophies treat
   `0000_00` safely as an additional campaign identity.

## Next smallest task

Perform the single backed-up-save test in
`dist/CompleteStory-v0.1/README.md`, then report the selector and launch result.
Do not expand the campaign graph until this gate passes.
