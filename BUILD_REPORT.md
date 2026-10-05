# Complete Story v0.1 — Offline Build Report

Build date: 2026-10-02

## Output

- `dist/CompleteStory-v0.1/CompleteStory_P.pak`
- `dist/CompleteStory-v0.1/CompleteStory_P.utoc`
- `dist/CompleteStory-v0.1/CompleteStory_P.ucas`
- `dist/CompleteStory-v0.1-Unverum.zip`

## Authored packages

1. Override:
   `/Game/SS/Blueprints/DragonAdventureIFData`
2. New asset:
   `/Game/SS/MasterDataAsset/DragonAdventureIF/CompleteStory/DAIF_CharaData_CompleteStory`

The registry has 13 `PtrRecords` entries and 29 imports. Records 1–12 match the
source serialization semantically and remain in their original order. Record
13 uses key `0000_00` and points to the new asset.

The new asset is a clone of Goku's character data with only its package/object
identity and `CharacterName` changed. `CharacterName` is culture-invariant text
`Complete Story`. Its verified start pointers are:

- `StartEvent`: `Event_00_0_00_00`
- `StartEventBlock`: `EventBlock_0000_00`
- `EventData`: `DIF_Event_0000_00`

## Offline checks passed

- Authorized key accepted by stock encrypted IoStore.
- Oodle runtime matched retoc's pinned SHA-256.
- Targeted source conversion: 2 assets, 0 failures.
- Exact build mapping parsed both unversioned assets.
- Source `.uexp` JSON round trips were byte-identical.
- Modified legacy binaries reparsed with all expected properties/imports.
- No existing registry entry was removed or replaced.
- Final `retoc verify`: passed.
- Final container: 2 packages, 3 chunks, indexed UE5.1 IoStore.
- Registry store entry: 13 imported route package IDs (12 original + 1 new).
- New character store entry: Goku's original 3 dependency package IDs.
- Full-context final decode: 0 failures and no unknown imports.
- Final decoded exports match authored exports; retoc normalized only the order
  of the registry's create-before-create dependency array, not its members.
- Distribution zip entries and component hashes verified.

## Not verified

- The game was not launched.
- A compiled UI/runtime limit of 12 entries cannot be excluded offline.
- New-key playability/save initialization has not been observed.
- No claim is made that the route is playable until the first user test.

## Reproducibility

The deterministic JSON transformation is in
`scripts/build_complete_story_assets.ps1`. It contains no encryption key.
