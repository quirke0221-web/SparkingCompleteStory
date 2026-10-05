# ADR 0004: Strict Ban on Transient Slate Widget Reflection in Runtime Scripting

## Status
Accepted

## Context
During experimental v0.6 and v0.7 iterations, developers attempted to unlock custom character campaigns and force UI menu selection by reflecting and polling transient Slate/UMG widgets across frames (e.g. running `FindAllOf("TextBlock")` or scanning widget hierarchies).

This pattern resulted in:
1. **Fatal Native Crashes (`0xC0000005` Access Violations):** In Unreal Engine 5, Slate widgets are pooled, destroyed, and recreated dynamically by the rendering pipeline. Holding or polling raw pointers across frames inevitably dereferences dangling or freed memory.
2. **Timing Sensitivity & Race Conditions:** UI element existence is frame-dependent and breaks across different monitor resolutions, aspect ratios, or menu transitions.
3. **Severe Performance Degradation:** Iterating all UMG/Slate widgets on GameThread introduces severe hitching and frame drops.

## Decision
1. **Strict Prohibition:** All RE-UE4SS Lua runtime scripts are **strictly FORBIDDEN** from reflecting, scraping, or polling transient Slate or UMG widget hierarchies across frames (e.g., `FindAllOf`, text scraping, or panel walking).
2. **Stable UFunction Interception:** Playability, character route unlock, and menu interaction must be managed exclusively through stable Unreal Engine data structures or native UFunction hooks on GameThread (such as hooking `IsPlayable` to return `true` for route key `0000_00`).
3. **Memory Safety Protocol:** Any runtime hook or state mutation must check `IsValid()` on target UObjects before executing logic.

## Consequences
* **Positive:** Complete elimination of `0xC0000005` access violation crashes caused by dangling Slate pointers.
* **Positive:** Deterministic, rock-solid campaign unlock behavior that operates identically across all game versions and framerates.
* **Negative / Compliance Requirement:** Future agent sessions in Codex or Antigravity must never attempt quick UI screen-scraping hacks and must follow the established UFunction hooking patterns in `.agents/skills/ue4ss-runtime-scripting/`.
