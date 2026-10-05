# Complete Story v0.1 — Episode Battle Prototype

This is a normal UE5.1 IoStore mod candidate. It does not use UE4SS or Lua.

## Included implementation

- Preserves the 12 original `DragonAdventureIFData.PtrRecords` entries.
- Appends a thirteenth key, `0000_00`.
- Adds a separate character data asset named
  `DAIF_CharaData_CompleteStory`.
- Displays that asset as **Complete Story** using culture-invariant text.
- Reuses Goku's original `Event_00_0_00_00`, `EventBlock_0000_00`, and
  `DIF_Event_0000_00`, which should start the canonical Raditz opening.

This package passed offline serialization, dependency, container, and
full-context decode checks. It has not been launched in the game, so menu
capacity, save initialization, and actual route launch are not yet proven.

## Files that must stay together

- `CompleteStory_P.pak`
- `CompleteStory_P.utoc`
- `CompleteStory_P.ucas`

## Manual installation

1. Back up your save before the first test.
2. Create a dedicated folder such as
   `SparkingZERO\Content\Paks\~mods\CompleteStory` in the game installation.
3. Copy the three `CompleteStory_P` files above into that folder.
4. Do not copy the obsolete UE4SS prototype.

To uninstall, remove only those three files (or their dedicated folder).

## Unverum

Import `CompleteStory-v0.1-Unverum.zip` as a Sparking ZERO mod, enable it, and
let Unverum place the three container files through the existing mod-loading
setup. This packaging layout has been prepared for Unverum but has not been
tested in-game.

## Minimum first test

1. Use a backed-up/disposable save and enable only the necessary mod loader plus
   Complete Story where practical.
2. Open Episode Battle and verify that all 12 original campaigns remain
   selectable and a thirteenth **Complete Story** entry appears.
3. Select Complete Story and verify that it begins Goku's original Raditz
   opening.
4. Stop after confirming the opening. Report the selector behavior and any log
   or load error before progressing further.
