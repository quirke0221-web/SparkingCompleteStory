---
name: ue4ss-runtime-scripting
description: Mandatory before ANY RE-UE4SS configuration or Lua script authoring for Dragon Ball Sparking! ZERO. Enforces 3.0.1 Beta APIs, bUseUObjectArrayCache configuration, safe GameThread execution, and strictly bans transient Slate/UMG widget reflection.
---

# Skill: RE-UE4SS Runtime Scripting

> **Status:** AUDITED & ACTIVE (Phase 3 Passed)  
> **Pinned version:** RE-UE4SS `v3.0.1 Beta #0`, Git SHA [`4e5461c`](https://github.com/UE4SS-RE/RE-UE4SS/tree/4e5461c0dd6201c654198a3e6f28126eaf9016fa)  
> (verified from local runtime receipt `evidence/runtime/v0.7-sanitized.log` lines 1-2)  
> **Reference folder:** [`docs/dependencies/ue4ss/`](../../../docs/dependencies/ue4ss/README.md)  
> (verbatim upstream receipts: `upstream-settings-ini.md`, `upstream-lua-hooks-threading.md`, `upstream-lua-lookup-params.md`, `upstream-creating-a-lua-mod.md`, `sparking-zero-guide.md`)

Every rule below cites its receipt. `[L:name]` = upstream Lua doc file, `[INI]` = `upstream-settings-ini.md`, `[SZ]` = `sparking-zero-guide.md`, `[LOCAL]` = local runtime logs/dumps. If an API is not in the pinned receipts, it does not exist in our build. Do not invent it (see `AGENTS.md` §1).

---

## 1. Scope Boundaries

**Use this skill for:**
- Authoring runtime Lua mods (`Mods\<ModName>\scripts\main.lua`).
- Registering native engine hooks via `RegisterHook` on `/Script/SS.*` functions.
- Configuring `UE4SS-settings.ini` parameters for stability in *Dragon Ball: Sparking! ZERO*.
- Scheduling GameThread actions via `ExecuteInGameThread`.

**Do NOT use this skill for (hand off instead):**
| Task | Owner skill |
|---|---|
| Unpacking / packing IoStore containers (`.pak`, `.utoc`, `.ucas`) | `retoc-iostore-packer` |
| Binary asset serialization / JSON editing | `uassetgui-asset-serialization` |
| Container staging, priority naming, and Unverum release packaging | `unverum-mod-packager` |
| End-to-end pipeline orchestration | `sparking-zero-mod-pipeline` |

---

## 2. Core Engine Invariants & Rules

### 2.1 Mandatory Settings Gate (`bUseUObjectArrayCache = false`)
- `[POLICY]` In `UE4SS-settings.ini`, always enforce:
  ```ini
  bUseUObjectArrayCache = false
  ```
- Upstream default is `true` (`[INI]`). The project's tested configuration (`evidence/runtime/UE4SS-settings-test.ini` line 24) and Unverum's bundled configuration (`docs/dependencies/unverum/bundled-ue4ss-settings-ini.md`) both disable this cache.
- `[HYPOTHESIS]` Leaving it enabled triggers access violations during stage or character loading in Sparking! ZERO.

### 2.2 Strict Zero Slate/UMG Scraping Ban (`AGENTS.md` §4.2)
- Slate and UMG UI widgets in Unreal Engine 5 are transient, recycled, and pooled in native memory.
- **FORBIDDEN:** Writing hooks or background loops that search for (`FindAllOf`, `FindFirstOf`), poll, or reflect Slate UI widgets (such as carousel panels or text blocks). Doing so causes fatal native access violations (`0xC0000005`) when the engine recycles or destroys widget memory across frames.
- State must be queried strictly through stable C++ game data structures or native function hook parameters.

### 2.3 Verified Class & Function Paths
- `[LOCAL]` Episode Battle Manager class is `/Script/SS.SSDragonAdventureIFCSManager` (module name is `SS`, **not** `SparkingZERO`; `evidence/object-dump/episode-battle-symbols.txt` line 178).
- `[LOCAL]` Function is `/Script/SS.SSDragonAdventureIFCSManager:IsPlayable` (lines 473-474). Its only declared parameter is `ReturnValue` (`BoolProperty`). No input parameters exist on this function.

### 2.4 Parameterless UFunction Return Value Override Bug (`LuaMod.cpp` line 315)
- `[FACT]` In RE-UE4SS `v3.0.1 Beta` (`4e5461c`), `RegisterHook` cannot override the return value of parameterless native functions (functions where `GetNumParms() == 1` and input parameter count is `0`, such as `IsPlayable` or `IsModeStart`).
- `[RECEIPT]` In `UE4SS/src/Mod/LuaMod.cpp` line 315, the loop setting `lua_data.return_property` is guarded by `if (has_properties_to_process && context.TheStack.Locals())`. For native parameterless functions, `TheStack.Locals()` is `nullptr`. Consequently, `lua_data.return_property` remains `nullptr`, causing `process_return_value()` (line 239: `else if (... && lua_data.return_property && context.RESULT_DECL)`) to silently evaluate to false and skip writing back the return value.
- `[FACT]` Calling `return_value:set(true)` in Lua also fails because primitive return types (such as `BoolProperty`) are pushed to Lua as raw primitive values (`boolean`), not `LocalUnrealParam` wrapper objects.
- `[POLICY]` Never rely on Lua `return <value>` or `return_value:set()` to override return values of parameterless native functions in UE4SS 3.0.1. Manipulate state directly via game data structures, reflected properties, or pre-hook arguments.

---

## 3. Verified Lua API & Hooking Patterns (Golden Paths)

### 3.1 Hook Registration: `RegisterHook`
```lua
-- Target UFunction must already exist in memory when RegisterHook is called.
-- For /Script/ functions: Callback 1 is pre-hook; optional Callback 2 is post-hook.
local preId, postId = RegisterHook(
    "/Script/SS.SSDragonAdventureIFCSManager:IsPlayable",
    function(self)
        -- Pre-callback logic
    end,
    function(self, returnValue)
        -- Post-callback logic
    end
)
```
- Returns `preId, postId` integer identifiers used to unregister (`UnregisterHook(name, preId, postId)`).
- `[WARNING]` Version restriction: APIs from UE4SS 4.x (`ExecuteInGameThreadWithDelay`, `RegisterLoadMapPreHook`, `CancelDelayedAction`) **do not exist** in build `4e5461c`.

### 3.2 Safe GameThread Execution: `ExecuteInGameThread`
```lua
ExecuteInGameThread(function()
    -- Executes safely on Unreal's GameThread via ProcessEvent as soon as game has time.
end)
```
- Use `ExecuteWithDelay(delayInMs, function)` for delayed execution (`[L:upstream-lua-hooks-threading.md]`).

---

## 4. Mod Directory Structure & Unverum Hazard

### 4.1 Standalone Mod Layout
```text
<GameRoot>\SparkingZERO\Binaries\Win64\Mods\
    <ModName>\
        scripts\
            main.lua
    mods.txt
```
In `mods.txt`, the mod must be explicitly enabled:
```text
<ModName> : 1
```

### 4.2 Unverum Build Wipe Hazard
- `[FACT]` Unverum's Build button calls `Restart` **before** building, which deletes `ue4ss.dll`, `dwmapi.dll`, `UE4SS-settings.ini`, `opengl32.dll`, `patternsleuth_bind.dll`, and the entire `Win64\Mods` folder (`docs/dependencies/unverum/source-receipts-build.md`).
- To make a Lua mod compatible with Unverum:
  - Place the mod inside a folder ending in `ue4ss` (e.g. `CompleteStory_ue4ss\CompleteStory\scripts\main.lua`).
  - Unverum detects `*ue4ss` folders, installs its bundled UE4SS binaries, copies the mod to `Win64\Mods`, and updates `mods.txt` automatically.

---

## 5. Pre-Flight Checklist (run BEFORE running Lua scripts)

- [ ] Installed build verified as `v3.0.1 Beta` / commit `4e5461c`.
- [ ] `UE4SS-settings.ini` has `bUseUObjectArrayCache = false`.
- [ ] Target function string verified against symbol dumps (module `SS`, not `SparkingZERO`).
- [ ] **Negative check:** Zero Slate widget reflection (`FindAllOf`, widget property reading) inside callbacks.
- [ ] **Negative check:** No 4.x-exclusive APIs used (`ExecuteInGameThreadWithDelay`, etc.).
- [ ] Mod directory structure contains `scripts\main.lua` and is enabled in `mods.txt`.

---

## 6. Post-Flight Checklist (run AFTER running)

- [ ] Check `UE4SS.log` for: `Registered native hook (PreId, PostId) for Function <Name>`.
- [ ] Verify game did not crash on startup or when entering Episode Battle.
- [ ] If running through Unverum, verify `Restart` did not wipe unbacked-up scripts from `Win64\Mods`.

---

## 7. Negative Constraints (Hallucination Defense)

| Forbidden | Why (receipt) | Do instead |
|---|---|---|
| Setting `bUseUObjectArrayCache = true` | Triggers memory crash hypothesis; tested config is `false` `[SZ:§1]` | Keep `false` |
| Scraping / polling Slate UI widgets (`FindAllOf("TextBlock")`) | Violates `AGENTS.md` §4.2; transient memory causes `0xC0000005` crash | Read stable engine data |
| Hooking `/Script/SparkingZERO...` | Module is `SS` `[LOCAL: episode-battle-symbols.txt:178]` | Hook `/Script/SS.SSDragonAdventureIFCSManager` |
| Adding dummy parameters to `IsPlayable` hook | Declared with only `ReturnValue` `[LOCAL: lines 473-474]` | Hook only `ReturnValue` |
| Calling `ExecuteInGameThreadWithDelay` | 4.x API; does not exist in build `4e5461c` `[SZ:WARNING]` | Use `ExecuteInGameThread` or `ExecuteWithDelay` |
| Putting development scripts directly in `Win64\Mods` | Unverum Build wipes `Win64\Mods` completely `[SZ:§4]` | Keep source in repo and stage for deployment |
| Relying on Lua `return <val>` to override parameterless native hooks | C++ bug in `LuaMod.cpp` L315: null `TheStack.Locals()` skips return write-back | Mutate game state or object properties directly |
| Calling `:set()` on primitive hook parameters (bool, int) | Pushed as primitive Lua types, not `LocalUnrealParam` wrapper | Check type and use direct assignment or object methods |
