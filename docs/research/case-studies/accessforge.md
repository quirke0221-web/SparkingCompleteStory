# Case Study: AccessForge (`SparkingZeroAccess`)

> **Subject:** [`AccessForge/SparkingZeroAccess`](https://github.com/AccessForge/SparkingZeroAccess)  
> **Domain:** Hybrid RE-UE4SS Architecture & Safe GameThread Interception  
> **Significance:** Primary blueprint for Complete Story's repository layout and runtime safety rules.

---

## 1. Executive Summary

AccessForge is a prominent community open-source accessibility mod for *Dragon Ball: Sparking! ZERO*. It provides screen-reader audio cues, battle menu narration, and UI feedback for visually impaired players. 

Because it operates at the intersection of external build automation and deep runtime memory hooking, AccessForge was audited during early research to resolve two critical project crises:
1. The `scripts/` vs `scripts/` directory naming collision.
2. The fatal `0xC0000005` Slate widget scraping crashes in UE4SS.

---

## 2. Key Architectural Discoveries

### A. Root Mod Structural Parity (ADR 0008)
* `[FACT]` AccessForge places its runtime mod directly at the repository root as `SparkingZeroAccess/` (containing `enabled.txt` and `scripts/main.lua`).
* `[OBSERVATION]` This establishes a bit-for-bit 1:1 match with the destination folder in the game installation:  
  `SparkingZERO\Binaries\Win64\Mods\SparkingZeroAccess\`
* `[POLICY]` Complete Story adopted this exact standard: eliminated the artificial `runtime/` wrapper and placed `CompleteStory/` directly at the repo root.

### B. Automation Isolation via `helpers/`
* `[FACT]` AccessForge isolates all external PowerShell build, asset extraction, and deployment cmdlets into a dedicated `helpers/` directory.
* `[OBSERVATION]` Prior to this, our project suffered severe confusion between root build scripts (`scripts/*.ps1`) and in-game Lua scripts (`CompleteStory/scripts/*.lua`).
* `[POLICY]` By renaming root `scripts/` to `helpers/`, developers and AI agents immediately know:
  - `helpers/` = Windows PowerShell build automation.
  - `CompleteStory/scripts/` = Live in-game Lua executing in engine memory.

### C. Safe Native Hooking & GameThread Concurrency
* `[FACT]` AccessForge initializes its hooks exclusively on the GameThread:
  ```lua
  ExecuteInGameThread(function()
      RegisterHook("/Script/...", hook_callback)
  end)
  ```
* `[OBSERVATION]` Unthreaded or premature hook execution during early engine boot causes heap corruption and startup crashes.

### D. Zero Slate Scraping (ADR 0004)
* `[FACT]` AccessForge queries native C++ actor and manager properties directly via protected calls (`pcall`). It never walks UMG or Slate UI widget hierarchies across frames.
* `[OBSERVATION]` Slate widgets (`TextBlock`, `Button`) are transient, pooled, and rendered on a separate rendering thread. Dereferencing them in native callbacks causes `0xC0000005` memory access violations (as suffered in our legacy v0.6/v0.7 prototypes).
* `[POLICY]` Codified as ADR 0004: All playability resolution in Complete Story queries stable manager state (`manager.SelectCharacterKey` / `CurrentCharacterData`), strictly banning Slate reflection.

---

## 3. Direct Impact on Complete Story

| AccessForge Architectural Pattern | Complete Story Adoption | Settled Decision |
| :--- | :--- | :--- |
| Root mod directory | `CompleteStory/` at repo root | [ADR 0008](../../ADRs/0008-accessforge-architectural-parity-helpers-and-root-mod.md) |
| External build tools in `helpers/` | Renamed root `scripts/` to `helpers/` | [ADR 0008](../../ADRs/0008-accessforge-architectural-parity-helpers-and-root-mod.md) |
| Safe GameThread initialization | `ExecuteInGameThread(init_mod)` in `main.lua` | [ADR 0007](../../ADRs/0007-runtime-scripting-module-consolidation-and-game-thread-safety.md) |
| Property-based state resolution | `resolve_focused_route_key()` via `pcall` | [ADR 0004](../../ADRs/0004-strict-ban-on-transient-slate-widget-reflection.md) |
