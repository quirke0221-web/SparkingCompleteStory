# CompleteStory13thTest v0.4

> **ABANDONED — DO NOT INSTALL OR RUN.** Complete Story has moved to a normal
> cooked-asset mod. This folder is retained only as historical evidence.

This is a controlled UE4SS prototype for testing whether the Episode Battle
selector consumes a thirteenth entry from `DragonAdventureIFData.PtrRecords`.
It is not the finished Complete Story campaign.

## Safety model

- `Ctrl+Alt+F8` is read-only and must pass before insertion is allowed.
- `Ctrl+Alt+F9` adds one transient map entry only after rechecking the 12-entry
  baseline, resolving an existing `0000_00` `FName`, and locating Goku's route
  by its exact data-asset path.
- `Ctrl+Alt+F10` removes the entry if this script inserted it in this session.
- Restarting the game also clears the change. No cooked asset is rewritten.
- The script does not intentionally call save APIs. The game itself may write
  progression if a route is entered, so the first visual test should stop at
  the selector unless a disposable/backed-up save is being used.

The temporary entry aliases Goku's existing character data. It may therefore
appear as a duplicate Goku entry. A separate `Complete Story` label and portrait
require a separately packaged character data asset or a verified UI hook.

## Install for user-run testing

Do not copy this while the game is running and do not reload all UE4SS mods.

1. Back up the currently installed `CompleteStory13thTest` directory.
2. With the game closed, replace only that mod directory with this directory:
   `SparkingZERO\Binaries\Win64\Mods\CompleteStory13thTest`
3. Launch the game normally when ready.
4. At the main menu, press `Ctrl+Alt+F8` once. Check `UE4SS.log` for
   `F8 PASSED`. If it does not pass, stop and send the Complete Story log lines.
5. Press `Ctrl+Alt+F9` once. Require `INSERTED` and a count of 13.
6. Open Episode Battle and inspect the selector. Do not start a route on the
   first pass. Record whether a thirteenth item exists, its placement, label,
   focus/navigation behavior, and whether all original entries remain.
7. Return to the prior menu and press `Ctrl+Alt+F10`. Require
   `ROLLBACK PASSED` and a count of 12. If rollback cannot be verified, close
   the game without further testing.

## Expected log prefix

All messages start with `[CompleteStory v0.4]` so the relevant lines can be
copied without sharing the full log.
