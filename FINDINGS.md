# Complete Story — Findings

Packaged-asset implementation attempt: 2026-10-02

## Packaging result

The earlier UE4SS prototype is retired. After the user supplied the authorized
key for this installed build, retoc successfully opened the encrypted base
container and performed targeted conversion of the registry and Goku character
data. The key is not stored in the workspace documentation, scripts, or build.

The retained FModel payload was not edited. retoc converted the packages from
the source IoStore with package-store metadata, UAssetAPI serialized the
modified legacy assets, and retoc rebuilt a UE5.1 IoStore overlay.

The result is `dist/CompleteStory-v0.1/CompleteStory_P.{pak,utoc,ucas}` plus
`dist/CompleteStory-v0.1-Unverum.zip`.

The final container:

- passes `retoc verify`;
- contains exactly the registry override and new character-data package;
- retains all 12 original registry entries and imported package IDs;
- adds one new package import and thirteenth map record;
- reparses in full base-container context without unknown imports; and
- retains Goku's Raditz start event, block, and event-data dependency.

This is offline verification only. The compiled selector capacity and save
initialization remain runtime gates.

## Earlier runtime-prototype outcome (archived)

The Episode Battle character list is a real data-asset registry and was found
to be mutable in principle through UE4SS's reflected `TMap` API. A guarded
workspace prototype was designed to test a thirteenth entry by aliasing the
existing Goku route under another already-valid character key. It did not
supply the final Complete Story label and was not run in game. This prototype
is no longer the implementation path.

## Corrected tool state

The previous documents stopped at an archive-access blocker. Later local work
resolved it:

- FModel `4.4.4.0` successfully mounted 76/81 containers, including the two
  encrypted base containers, and indexed 412,783 files.
- After loading `Mappings.usmap`, FModel opened the registry, Goku character
  data, event data, chart data, two opening event blocks, the chart selector
  Blueprint, and the character-selection widget.
- The mapping used by that session is 2,369,253 bytes with SHA-256
  `B7AE00F54BA558EF7793CABA3437B3E82D64D29A6822C34CE73B4A1F91B2D1C5`.
  Its practical compatibility with these targeted packages is verified by
  successful property parsing. Its broader completeness is not assumed.
- A different temporary mapping from SparkingZeroCssOrganizer is 2,339,777
  bytes with SHA-256
  `51AF924FF6779EA9E24129E6EDBD27AD718AC8E4289FB908504FBACCDCE9D675`.
  It differs byte-for-byte and was not treated as the mapping used for these
  findings.
- The only retained minimal raw export is
  `tools/FModel/FModel-aug-2026/Output/Exports/SparkingZERO/Content/SS/Blueprints/DragonAdventureIFData.uasset`.
  No full extraction was performed.

## Registry and key layout

Runtime reflection from the current game build confirms:

- `SSDragonAdventureIFDataAsset::PtrRecords` at offset `0x40` is a `MapProperty`.
- Its key is the script struct `KoratCharacterDataList`.
- `KoratCharacterDataList` contains one `NameProperty` named `Key` at offset 0.
- Its value is an `SSDragonAdventureIFCharacterDataAsset` object pointer.
- The registry also contains `DefaultOpenCharacter`, `CameraSequencer`,
  `BGMDataList`, `UIAssetArray`, and debug override fields.

This makes the v0.3 table form `{ Key = FName(...) }` structurally reasonable.
The unresolved part was not the struct field layout; it was whether the assumed
key was correct, whether the map API was being used on the installed UE4SS
build, and whether an arbitrary new `FName` was acceptable.

The current UE4SS documentation defines `#map`, `ForEach`, `Contains`, `Find`,
`Add`, and `Remove` for `TMap`. The local UE4SS build and shared helpers also
expose `StaticFindObject`, `FName(..., EFindName.FNAME_Find)`, remote parameter
`get()`, and `UObject:GetFullName()`. References:

- [UE4SS TMap API](https://github.com/UE4SS-RE/RE-UE4SS/blob/main/docs/lua-api/classes/tmap.md)
- [UE4SS StaticFindObject API](https://docs.ue4ss.com/lua-api/global-functions/staticfindobject.html)
- [UE4SS v3.0.1 release](https://github.com/UE4SS-RE/RE-UE4SS/releases/tag/v3.0.1)

## Cause and correction of the failed prototype

The logged `Read failed safely: function: ...` belongs to an older script. It
does not contain the actual thrown error because that version propagated/logged
the wrong `pcall` value. The installed v0.3 file was written after the last log
entry, so v0.3 itself never ran and cannot be described as failed or passed.

The offline correction is therefore evidence-driven rather than a fabricated
single root cause:

- Errors are now logged at the exact operation with the actual `pcall` error.
- Registry lookup uses the verified full object path with a class-instance
  fallback.
- `PtrRecords` is enumerated with the documented remote-parameter unwrapping.
- Goku is located by the exact data-asset path, eliminating the disputed
  hardcoded `0000_40` lookup assumption.
- The insertion key is `0000_00`, already known to be a real character-data key,
  and is resolved with `FNAME_Find`. The script blocks if it is not already in
  the name pool or is already a route key.
- No mutation can occur until the read-only pass observes the exact expected
  baseline.

Only a game-run F8 can identify any remaining API/build-specific failure.

## How a campaign is registered and launched

The verified data chain is:

`DragonAdventureIFData.PtrRecords key`
→ `SSDragonAdventureIFCharacterDataAsset`
→ `StartEventBlock` + `StartEvent` + `EventData`
→ matching event record
→ battle/story/time-slice payload
→ next-event and unlock maps.

For Goku, prior FModel inspection identified:

- Character data:
  `/Game/SS/MasterDataAsset/DragonAdventureIF/0000_00/DAIF_CharaData_0000_00`
- Start event: `Event_00_0_00_00`
- Start block: `EventBlock_0000_00`
- Event data: `DIF_Event_0000_00`
- Opening chart assets include
  `EventBlock_0000_40_0000_00` and `EventBlock_0000_40_0000_01`.

The exact controller bytecode that converts the selected registry key into the
travel request was not decompiled. Native reflection shows the
`SSDragonAdventureIFCharacterSelectController`, its character manager and select
menu, and the `SSDragonAdventureIFCSManager::IsPlayable/IsModeStart` functions.
The route asset fields above are the verified data handoff into progression.

## Menu capacity

The selector widget is
`/Game/SS/UI/AdventureIF/WBP_GRP_AI_CharacterSelect`.

Its reflected widget class contains exactly six reusable character-panel
objects (`WBP_OBJ_AI_CharacterPanel_0` through `_5`), three menu buttons, and
loop/array operations in `SetVisibleOnContinue` and its event graph. This is
consistent with pages or recycled panels rather than twelve permanent slots.
It does not prove a third page or thirteenth item works. No reflected constant
of 12 was found, but compiled Blueprint control flow was not fully decompiled.

Answer: registry length is dynamic; final rendered capacity/navigation is still
runtime-unverified.

## Separate Complete Story data asset

An additional route can be represented by another registry map entry pointing
to a separate `SSDragonAdventureIFCharacterDataAsset`. That asset owns the
display name, introduction, presentation actors/sequences, starting event/block,
event data, route-clear info, and related settings. Therefore the clean design
is an additive `CompleteStory` character data asset that initially copies
Goku's route references, not a mutation of Goku's stock object.

UE4SS can test the registry behavior with an alias, but it cannot safely create
and persist the complete cooked asset graph used by the shipping game. The final
asset should be cooked with the established Sparking ZERO mod project/custom
UE5 workflow and deployed as a removable overlay after the alias test passes.

## Save/progression impact

Current-build reflection confirms:

- `FSSDragonAdventureIFSaveData::CharacterData` is a map keyed by
  `FKoratCharacterDataList`.
- `CharacterPlayableData` is a second map with the same key type.
- Per-character state includes name-keyed event-block and event maps, last block,
  unlock data, and activity state.
- Per-event state includes result index, unlock state, normal/alternate clear
  counts, and reward state.
- Route-clear records can participate in original-route and trophy checks.

Thus storage is structurally extensible but initialization semantics are not
proven. A new key may be lazily initialized, rejected as unplayable, omitted
from saves, or included in totals. The first test must inspect only the selector;
route entry comes later with a disposable/backed-up save.

## Offline validation performed

- Read the current workspace and installed prototype; no live file was changed.
- Correlated FModel logs, retained export, mapping hashes, UE4SS log, shared Lua
  API definitions, and the 202 MB current-build object dump.
- Confirmed every UObject/property/API name used by v0.4 exists in local
  reflection or local UE4SS helpers.
- Reviewed the Lua control flow for guarded mutation and rollback.
- No standalone Lua interpreter is installed, so bytecode compilation was not
  available without adding another tool. No tool was installed.
- No runtime or visual claim is made.

## Concrete blocker

Offline work cannot prove that the compiled character-selection Blueprint
renders a thirteenth registry record, nor can it observe save initialization.
The required next evidence is the bounded user-run F8/F9 selector test described
in `mods/CompleteStory13thTest/README.md`.
