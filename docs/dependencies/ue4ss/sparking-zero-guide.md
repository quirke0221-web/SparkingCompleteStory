# `RE-UE4SS` Usage Guide for Dragon Ball: Sparking! ZERO

**Installed build (receipt):** `UE4SS - v3.0.1 Beta #0 - Git SHA #4e5461c` and `Found EngineVersion: 5.1`
(`evidence/runtime/v0.7-sanitized.log` lines 1-2).
**Reference docs:** pinned to that exact commit, `4e5461c`. See [`README.md`](README.md).

Labels (see `AGENTS.md` §1.3):
- `[FACT]` = pinned upstream doc.
- `[LOCAL]` = evidence in this repo.
- `[POLICY]` = a project rule.
- `[OBSERVATION]` = something seen at runtime, cause unproven.
- `[HYPOTHESIS]` = unproven.

> [!WARNING]
> Docs at `docs.ue4ss.com/dev` describe **UE4SS 4.x**. Functions such as
> `ExecuteInGameThreadWithDelay` and `RegisterLoadMapPreHook` are **not documented at
> `4e5461c`**. Use only the pinned docs in this folder.

---

## 1. Settings

- `[FACT]` Upstream default is `bUseUObjectArrayCache = true`. Its comment: "Setting this to false can
  help if you're experiencing a crash on startup" ([`upstream-settings-ini.md`](upstream-settings-ini.md)).
- `[LOCAL]` The project's tested config sets it to `false`
  (`evidence/runtime/UE4SS-settings-test.ini` line 24).
- `[FACT]` Unverum's bundled UE4SS settings also set it to `false`
  ([`../unverum/bundled-ue4ss-settings-ini.md`](../unverum/bundled-ue4ss-settings-ini.md)).
- `[POLICY]` Use `bUseUObjectArrayCache = false`.
- `[HYPOTHESIS]` Leaving it `true` crashes Sparking! ZERO. Not tested in this project. The earlier
  "GUObjectArray is reallocated during battle loading" explanation had **no source** and was removed.

## 2. Hook Target Facts

- `[LOCAL]` The Episode Battle manager class is `/Script/SS.SSDragonAdventureIFCSManager`. The game
  module is `SS`, **not** `SparkingZERO` (`evidence/object-dump/episode-battle-symbols.txt` line 178).
- `[LOCAL]` `IsPlayable` is `Function /Script/SS.SSDragonAdventureIFCSManager:IsPlayable`, and the only
  property listed under it is `ReturnValue` (BoolProperty) (same file, lines 473-474). No input parameters appear.
- `[LOCAL]` UE4SS registered a native hook on that exact path
  (`v0.7-sanitized.log`: `Registered native hook (1, 2) for Function /Script/SS.SSDragonAdventureIFCSManager:IsPlayable`).
- `[FACT]` `RegisterHook`: the target `UFunction` "must already exist in memory". For `/Script/` paths,
  callback 1 runs before and the optional callback 2 runs after. Returns `PreId, PostId`
  ([`upstream-lua-hooks-threading.md`](upstream-lua-hooks-threading.md)).
- `[FACT]` `ExecuteInGameThread` executes "using `ProcessEvent`" and runs "as soon as the game has time".

## 3. Crash History (what is and is not known)

| Version | What the hook did | Result | Known cause |
|---|---|---|---|
| v0.6 | Hooked `IsPlayable`/`DecideButton`/`NewDecideButton`; widget-text and global searches in callbacks | Hooks registered, then Episode Battle crashed | `[OBSERVATION]` "Exact native crash site was not proven" (`docs/VERSION_HISTORY.md`) |
| v0.7 | One `IsPlayable` hook, delayed 60 s; read `Index`/`ShowNum` from six panel widgets inside the callback | Logged `-3/6,-2/4,-1/2,1/2,2/4,3/6`, then crashed | `[OBSERVATION]` Crash followed panel reflection inside the callback. Root cause unproven |

- `[LOCAL]` The panel values are relative carousel positions, not registry indexes (`VERSION_HISTORY.md` v0.7).
- `[POLICY]` (from `AGENTS.md` §4.2) Do not search, poll, or reflect UI widgets (`FindAllOf` on widget
  classes, panel properties) inside hooks or loops. Read state from stable game data instead.
- `[HYPOTHESIS]` Widget pooling or recycling caused the crashes. Not proven. Treat it as the leading theory only.

## 4. Unverum Interaction (important)

- `[FACT]` The Build button calls `ModLoader.Restart` **before** `ModLoader.Build`
  (`MainWindow.xaml.cs` L827 and L844, in [`../unverum/source-receipts-build.md`](../unverum/source-receipts-build.md)).
- `[FACT]` `Restart` **deletes** `ue4ss.dll`, `dwmapi.dll`, `UE4SS-settings.ini`, `opengl32.dll`,
  `patternsleuth_bind.dll`, the `Win64\Mods` folder, and `LogicMods` (`ModLoader.cs` L18-L74).
- `[FACT]` `Build` then reinstalls **Unverum's own bundled UE4SS** only if an enabled mod contains a folder
  whose name ends in `ue4ss` **or** `LogicMods` (`ModLoader.cs` L311, L323, L455-L483).
- `[FACT]` Consequence: any of those listed files from a manual UE4SS install are deleted on every Unverum build.
  Which proxy DLL our manual install uses is not recorded here, so whether it is in that list is unverified.
- `[HYPOTHESIS]` Unverum's bundled UE4SS may differ from the `4e5461c` build we tested. Its version is unknown.

## 5. Verified Post-Hook Return Value Limitations
- `[FACT]` `IsPlayable` and `IsModeStart` are parameterless native functions (`GetNumParms() == 1` with 0 input parameters).
- `[FACT]` In RE-UE4SS `4e5461c`, `RegisterHook` post-callback `return <value>` cannot override return values of parameterless native functions due to `context.TheStack.Locals()` being null in `LuaMod.cpp` line 315, which prevents `lua_data.return_property` assignment and causes `process_return_value()` (line 239) to skip write-back.
- `[FACT]` Boolean return values are passed to post-callbacks as raw Lua booleans, not objects with `:set()`. Calling `:set()` on them is invalid.
- `[POLICY]` Control flow and playability must be managed by mutating live game data structures or object properties directly, not by relying on post-hook return value overrides.

## 6. Not Yet Verified (do not present as fact)
- Exact property path in memory connecting `WBP_GRP_AI_CharacterSelect_C` or `SSDragonAdventureIFCSManager` to active character unlock display.

