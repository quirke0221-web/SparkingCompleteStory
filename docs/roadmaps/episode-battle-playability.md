# Complete Story Episode Battle Playability Plan

## Goal and scope

Make the separate `Complete Story` entry offer `New Game` and later `Continue`, launch its intended opening, and preserve all 12 stock campaigns and vanilla save data. This plan addresses the current `Unlock` label and NEO store dialog. It does not assume that the dialog proves a missing DLC license.

The work is confined to the authoritative runtime mod in `CompleteStory/`, its `helpers/` build and deployment workflow, and evidence-backed campaign assets if a traced native prerequisite requires an asset change. Do not alter stock DLC tables or vanilla save files.

## Evidence baseline

- [OBSERVATION] The deployed thirteenth tile appears, says `Unlock`, and opens the NEO storefront dialog on confirmation. The 2026-10-06 playtest reproduces the v0.3 result in [`legacy-iteration-history.md`](../research/legacy-iteration-history.md).
- [FACT] The asset build adds `0000_00` to both character and chart registries and retains the stock `DefaultOpenCharacter = 0000_40`; see [`Transform-CompleteStoryAssets.ps1`](../../helpers/Transform-CompleteStoryAssets.ps1) and [`asset-data-dictionary.md`](../research/asset-data-dictionary.md).
- [FACT] The local object dump shows `CharacterPlayableData`, its `UnlockInfo` value, `SSBuiltInMenu` decision functions, and a parameterless `SSDragonAdventureIFCSManager:IsPlayable` with a Boolean return; see `evidence/object-dump/targeted-symbols.txt` in the legacy project evidence snapshot. That dump does not show native function bodies or control flow.
- [OBSERVATION] The current UE4SS log records registration of the `IsPlayable` hook. It does not establish whether the hook ran during selection: the current callback logs only when its resolved key changes, and both its initial key and an unresolved key are `nil`; see [`main.lua`](../../CompleteStory/scripts/main.lua).
- [DISPROVEN] Appending `0000_00` to DLC 013 `AdventureIFCharacterIds` did not change the `Unlock` label or store dialog; see [`legacy-iteration-history.md`](../research/legacy-iteration-history.md). This disproves that edit, not every possible entitlement check.
- [HYPOTHESIS] The custom registry key lacks native playable state. `docs/ARCHITECTURE.md` in the legacy project snapshot notes this is consistent with observed behavior, while leaving the exact store routing unresolved.

## Decision gates

### 1. Identify the first divergent native decision

Research the pinned RE-UE4SS 3.0.1 references and existing game object dump before adding any probe. Record the selected campaign key, hook invocation count, original playability result, and menu decision path for one known playable stock campaign and `0000_00`. Use stable manager data or verified UFunction parameters on the GameThread. Log only bounded, non-sensitive values; do not read Slate/UMG widgets, scan all objects in a hot callback, or modify return values during this trace.

Investigate the relevant `SSBuiltInMenu` decision and `SSDragonAdventureIFCSManager` functions only after confirming their signatures and callback safety. Save a sanitized comparison in `docs/research/` with source or runtime receipts. A register-hook message alone is insufficient evidence that a callback fired.

**Exit gate:** A trace identifies the selected key and the earliest proven point where stock and Complete Story take different paths. If key identity cannot be obtained safely, stop implementation and research another stable game-owned source. Do not infer the key from tile text or carousel position.

### 2. Determine the authoritative playability model

Use the trace to decide which model the game actually uses for this menu:

| Finding | Implementation direction to investigate |
| --- | --- |
| `IsPlayable` is called for `0000_00` and returns false | Trace the underlying native state read. Determine whether a custom-key playable record can be supplied safely and whether the same state drives label and confirmation. |
| Menu rejects `0000_00` before `IsPlayable` | Identify that earlier campaign-recognition or entitlement decision and its game-owned inputs. Integrate the custom key there; a later `IsPlayable` override would not address the root. |
| `IsPlayable` succeeds but confirmation opens the store | Trace the menu's confirm dispatch and chart/start prerequisites; fix the first proven misroute. |
| A legitimate entitlement check is proven for this exact key | Document the verified check and find a lawful custom-content path. Do not forge DLC ownership or alter stock entitlements. |

**Exit gate:** A receipt-backed cause-and-effect account explains both `Unlock` and the store action. Update the research note before changing behavior. If the evidence changes the approved architecture, align the relevant PRD/architecture decision before implementation.

### 3. Implement one coherent custom-campaign path

Implement the smallest game-owned integration proven by gate 2 so `0000_00` has a consistent identity through selection, playable state, confirmation, and launch. Keep stock keys on the game's existing path. Store custom progress separately or use an explicitly verified safe mechanism that cannot overwrite or invalidate `MainGameSaveData`; decide this only after tracing save access and lifecycle. Add a `New Game`/`Continue` state model rather than changing display text alone.

Use the active `CompleteStory/scripts/main.lua` and approved dependency skills for runtime work. If verified asset references are missing, change only the relevant assets through `helpers/Transform-CompleteStoryAssets.ps1` and the existing build pipeline. Keep each script at or below 300 lines. Do not change `DefaultOpenCharacter`, `IgnoreOpen`, stock DLC records, or stock campaign records to force entry.

**Exit gate:** With a clean user test profile, Complete Story shows `New Game`; confirmation reaches the intended Raditz opening without a storefront dialog. A subsequent test shows `Continue` only when custom progress exists.

### 4. Verify safety, persistence, and deployment

Run PowerShell syntax checks, focused runtime checks, the full `helpers/Build-CompleteStory.ps1` pipeline, `retoc verify`, release archive audit, and `helpers/Deploy-DevelopmentBuild.ps1`. Compare all 12 stock campaign entries before and after installation. Playtest title-screen boot, stock Episode Battle selection, Complete Story new start, exit/restart/resume, and uninstall/reinstall behavior. Verify the vanilla save remains valid before and after removing the mod. Inspect UE4SS/game logs and crash reports after each playtest.

**Done when:** `New Game` and `Continue` behave correctly, the Raditz opening launches, no store prompt or crash occurs, all stock campaigns remain playable, vanilla save integrity is preserved, and the verified changes are committed and pushed under the repository workflow.

## Execution order and stop rules

1. Commit the read-only diagnosis and sanitized receipts separately from runtime code.
2. Present the proven native decision and the proposed player-facing fix before modifying code or assets, as required by `AGENTS.md` section 2.2.
3. Implement and verify one dependency at a time; commit and push each verified domain change.
4. If a probe crashes, a key source is unstable, or a result is ambiguous, revert the experimental commit and return to research. A visible `New Game` label without successful launch is not completion.
