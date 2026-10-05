# Sparking ZERO: Complete Story

> **Authoritative status (2026-10-04):** v0.3 is the clean proven menu/asset baseline. The thirteenth entry appears in game but remains locked and routes to an unrelated NEO storefront. v0.7 crashes after unsafe panel reflection. No Complete Story Raditz launch or cross-character transition has been verified. Begin with [START_HERE.md](START_HERE.md); older sections below are retained as development history.

Development project for adding a separate Complete Story option to Episode Battle while preserving the original 12 campaigns.

## Current verified state

- The cooked v0.3 asset baseline adds a visible 13th **Complete Story** selection and preserves the original campaigns.
- The character and chart registries contain the custom `0000_00` mapping, with the chart reusing Goku's `ChartData0000_00` and canonical Raditz opening references.
- The game treats the custom campaign as locked because it lacks a native `CharacterPlayableData` state. Confirming it can invoke the unrelated NEO storefront fallback.
- No released build currently provides a verified playable Complete Story campaign.

## Current runtime experiment

The active experiment is `runtime/v0.7/CompleteStory/Scripts/main.lua`.

Delayed installation of the `SSDragonAdventureIFCSManager::IsPlayable` hook prevents the earlier startup exit. Entering Episode Battle still crashes after the callback inspects the six character-panel objects.

The recorded panel values were:

```text
-3/6,-2/4,-1/2,1/2,2/4,3/6
```

These are relative carousel positions, not campaign registry indices. Comparing them with registry index `12` is invalid. The v0.7 script is retained as a failing diagnostic reproduction, not as a working release.

## Next development task

Remove all panel-object reflection from the `IsPlayable` callback and identify the selected campaign through stable manager/native state. Do not restore the old F8/F9 hotkey prototype or globally override playability.

## Repository scope

This repository contains source code, build scripts, metadata inventories, and investigation notes. It intentionally excludes extracted game assets, cooked packages, saves, AES keys, third-party executables, build output, and ZIP distributions.

Older investigation documents describe the state at the time they were written. This README is the authoritative current status.

## Complete development history

All project-created Lua prototypes and runtime experiments are retained. Superseded implementations are under `archive/legacy`, with their original files intact and clearly labeled so they cannot be confused with the current experiment.

Packaging metadata and release notes are retained under `packaging` and `archive/release-notes`. Generated containers and extracted/cooked game assets remain local-only.
