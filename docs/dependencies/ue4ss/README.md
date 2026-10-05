# `RE-UE4SS` Dependency Knowledge Base

* **Tool:** `RE-UE4SS` `v3.0.1 Beta`, Git SHA [`4e5461c`](https://github.com/UE4SS-RE/RE-UE4SS/tree/4e5461c0dd6201c654198a3e6f28126eaf9016fa)
  (the build actually installed, per `evidence/runtime/v0.7-sanitized.log` lines 1-2)
* **Authors:** UE4SS-RE Team
* **Repository:** [UE4SS-RE/RE-UE4SS](https://github.com/UE4SS-RE/RE-UE4SS)
* **Role in Project:** Runtime injection and Lua scripting for Unreal Engine 5
* **Agent Skill:** `.agents/skills/ue4ss-runtime-scripting/` (not yet written; Phase 3)

> [!WARNING]
> `docs.ue4ss.com/dev` documents UE4SS **4.x**, not our build. Use only the pinned copies below.

---

## Reference Documents in this Folder

| File | What it is | Trust level |
|---|---|---|
| [`raw-readme.md`](raw-readme.md) | Upstream README @ `4e5461c`, verbatim | Primary |
| [`upstream-settings-ini.md`](upstream-settings-ini.md) | Default `UE4SS-settings.ini` @ `4e5461c`, verbatim | Primary |
| [`upstream-lua-hooks-threading.md`](upstream-lua-hooks-threading.md) | Lua API: `RegisterHook`, `UnregisterHook`, `ExecuteInGameThread`, `ExecuteWithDelay`, `ExecuteAsync`, `LoopAsync`, `NotifyOnNewObject` | Primary |
| [`upstream-lua-lookup-params.md`](upstream-lua-lookup-params.md) | Lua API: `FindFirstOf`, `FindAllOf`, `StaticFindObject`, `RemoteUnrealParam`, `LocalUnrealParam`, `UFunction`, `Mod` | Primary |
| [`upstream-lua-uobject.md`](upstream-lua-uobject.md) | Lua API: `UObject` class | Primary |
| [`upstream-creating-a-lua-mod.md`](upstream-creating-a-lua-mod.md) | Guide: creating a Lua mod | Primary |
| [`upstream-installation-guide.md`](upstream-installation-guide.md) | Installation guide | Primary |
| [`sparking-zero-guide.md`](sparking-zero-guide.md) | Project facts, labeled `[FACT]`/`[LOCAL]`/`[POLICY]`/`[OBSERVATION]`/`[HYPOTHESIS]` | Derived |

All primary files are script-generated from a clone at `4e5461c`. Do not hand-edit them.
If a derived doc and a primary file disagree, the primary file wins. Fix the derived doc.

---

## Correction Log

**2026-10-05:** The first drafts were audited and these problems were fixed:
1. `lua-api-reference.md` and `configuration-reference.md` were deleted. They were retyped from memory and
   mixed 4.x APIs (`ExecuteInGameThreadWithDelay`, `RegisterLoadMap*`, `CancelDelayedAction`) into a 3.0.1 build.
2. The hook path `/Script/SparkingZERO...` and a `CharacterId` parameter were **[DISPROVEN]**. The real path is
   `/Script/SS.SSDragonAdventureIFCSManager:IsPlayable` with only a `ReturnValue` (object dump, lines 178, 473-474).
3. "`bUseUObjectArrayCache = false` is mandatory because GUObjectArray is reallocated" had no source.
   The setting is kept as `[POLICY]`; the reason is now `[HYPOTHESIS]`.
4. The crash history was relabeled: causes are `[OBSERVATION]`/`[HYPOTHESIS]`, not facts.

---

## Agent Skill Status
* **Status:** Not started. Receipts are ready; skill is Phase 3 of the implementation plan.
