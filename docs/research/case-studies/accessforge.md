# Case Study: AccessForge (`SparkingZeroAccess`)

> **Subject:** [`AccessForge/SparkingZeroAccess`](https://github.com/AccessForge/SparkingZeroAccess) (Audited @ `main`)  
> **Domain:** Hybrid RE-UE4SS Mod Architecture & In-Engine Runtime Memory Interception  
> **Target Game:** *Dragon Ball: Sparking! ZERO* (Unreal Engine 5.1.1, Steam PC)  
> **Significance:** Primary architectural blueprint for Complete Story's repository layout, GameThread concurrency, and Slate reflection avoidance.

---

## 1. Executive Summary & Problem Context

AccessForge is an open-source screen-reader accessibility mod for *Dragon Ball: Sparking! ZERO*. It reads battle HUD metrics (HP, Ki, Sparking meters), character roster grids (208 fighters), menu labels, and dialog prompts aloud via NVDA, JAWS, or Windows SAPI.

Operating inside a closed, pre-compiled Unreal Engine 5 binary without source code presents extreme architectural challenges:
1. **Host Monolith Ownership:** The engine controls the main loop, memory layout, rendering thread, and garbage collector.
2. **Namespace Collision:** External build/extraction scripts and in-game Lua scripts both fight for the `scripts/` directory name.
3. **Transient Memory Traps:** UI widgets in UE5 are transient and pooled across rendering frames, causing fatal native access violations (`0xC0000005`) if inspected unsafely.

AccessForge solved all three challenges through disciplined repository scaffolding and defensive Lua architecture.

---

## 2. Full Repository Directory Anatomy

The `AccessForge/SparkingZeroAccess` repository enforces a strict boundary between code executing **inside the game process** and maintenance tools executing **outside the game on Windows**:

```text
SparkingZeroAccess/
├── SparkingZeroAccess/            <-- Authoritative In-Game Mod Root (Deployed to game)
│   ├── enabled.txt                <-- RE-UE4SS mod activation flag
│   └── Scripts/                   <-- In-Game Lua Modules (Executed by UE4SS Lua 5.4)
│       ├── main.lua               <-- Master orchestrator: lifecycle, keybinds, hook init
│       ├── helpers.lua            <-- Defensive circuit-breaker wrappers (TryCall, pcall)
│       ├── speech.lua             <-- Screen-reader audio dispatch (NVDA/JAWS/SAPI)
│       ├── widget_reader.lua      <-- Text reading, widget matching, label resolution
│       ├── poll_trackers.lua      <-- State polling loops (dialogs, help windows, rooms)
│       ├── icon_parser.lua        <-- Rich text icon markup parser (converts icons to words)
│       ├── battle.lua             <-- Battle HUD monitor (HP, Ki, Sparking gauge deltas)
│       └── chara_names.lua        <-- Character texture-ID to display-name lookup table
│
├── scripts/                       <-- Maintenance Automation (Executed on Windows outside game)
│   └── Update-CharaNames.py       <-- Python script to extract DLC texture IDs from new game updates
│
├── LICENSE                        <-- MIT License
└── README.md                      <-- Installation guide, keybinds, and mod features
```

---

## 3. Architectural Scaffolding & Design Patterns

AccessForge decomposes its runtime responsibilities across four distinct architectural patterns:

### A. The Modular Subsystem Pattern (`require()` Decomposition)
* `[FACT]` Rather than writing a monolithic 1,000-line script, `main.lua` acts as a lightweight orchestrator:
  ```lua
  local helpers = require("helpers")
  local speech = require("speech")
  local battle = require("battle")
  ```
* `[OBSERVATION]` Each domain concern (speech output, battle HUD math, widget inspection, icon parsing) resides in an isolated, testable single-responsibility module.

### B. The Defensive Circuit-Breaker Pattern (`helpers.lua`)
* `[FACT]` Direct native pointer dereferencing in Lua (`object.Property` or `object:Function()`) throws fatal uncatchable engine exceptions if the underlying C++ UObject has been garbage collected or invalidated by UE5.
* `[FACT]` AccessForge wraps all native engine interactions in defensive wrappers:
  ```lua
  function TryCall(object, func_name, ...)
      if not IsValidRef(object) then return nil end
      local ok, result = pcall(function(...) return object[func_name](object, ...) end, ...)
      if ok then return result else return nil end
  end
  ```
* `[POLICY]` In Complete Story, we adopted this exact pattern in `CompleteStory/scripts/main.lua` via `is_valid()` and `unwrap()`, ensuring that dangling pointers return `nil` rather than crashing the game.

### C. The Concurrency & Thread Boundary Pattern
* `[FACT]` RE-UE4SS initializes in a secondary thread during early process creation. Registering hooks or accessing Unreal's `GUObjectArray` before the game engine's main thread stabilizes causes heap corruption.
* `[FACT]` AccessForge wraps all hook installations in `ExecuteInGameThread()`:
  ```lua
  ExecuteInGameThread(function()
      RegisterHook("/Script/...", hook_callback)
  end)
  ```
* `[POLICY]` Complete Story strictly enforces this invariant: `main.lua` executes its registration exclusively via `ExecuteInGameThread(init_mod)`.

### D. The Reflection Boundary: Stable State vs. Slate Scraping
* `[FACT]` Unreal Engine 5 separates UMG widgets (high-level Blueprint objects) from Slate widgets (low-level C++ rendering primitives). Slate widgets are pooled, transient, and rendered on the Slate thread.
* `[OBSERVATION]` Polling or reflecting Slate text blocks across frames (`FindAllOf("TextBlock")`) dereferences recycled memory addresses, resulting in `0xC0000005` access violations (the exact root cause of Complete Story's legacy v0.6/v0.7 crashes).
* `[POLICY]` AccessForge restricts polling to stable actor and manager pointers on the `GameThread`. Complete Story codified this as [ADR 0004](../../ADRs/0004-strict-ban-on-transient-slate-widget-reflection.md): all campaign unlock checks query native manager pointers (`SelectCharacterKey`), completely banning UI screen-scraping.

---

## 4. End-to-End Execution Lifecycle Trace

```mermaid
sequenceDiagram
    autonumber
    actor Player
    participant Steam as Steam (SparkingZERO.exe)
    participant UE4SS as RE-UE4SS (Win64\ue4ss.dll)
    participant Main as SparkingZeroAccess\main.lua
    participant Engine as UE5 GameThread / C++ Managers
    participant ScreenReader as NVDA / Windows SAPI

    Player->>Steam: Launches Game via Steam
    Steam->>UE4SS: Injects ue4ss.dll via proxy DLL (dsound.dll)
    UE4SS->>Main: Detects enabled.txt, loads Scripts\main.lua
    Main->>Engine: Dispatches ExecuteInGameThread(init_mod)
    Engine->>Main: GameThread acknowledges; registers native UFunction hooks
    loop Active Gameplay (Battle / Menus)
        Engine->>Main: Hook triggers on state change (e.g. Ki spent, menu moved)
        Main->>Main: Validates object pointers via helpers.TryCall() (pcall)
        Main->>ScreenReader: Emits sanitized text strings
        ScreenReader->>Player: Audio announcement plays in player's headphones
    end
```

---

## 5. Transferable Rules for Future Mod Projects

When building any Unreal Engine runtime mod using RE-UE4SS, obey these architectural laws:

1. **Root Structural Parity:** Name the mod repository folder identically to its destination in `Win64\Mods\<ModName>\` to provide 1:1 mental clarity ([ADR 0008](../../ADRs/0008-accessforge-architectural-parity-helpers-and-root-mod.md)).
2. **Helpers vs Scripts Namespace Law:** Never put external Windows build/maintenance scripts in a root `scripts/` folder if the mod itself has an in-game `scripts/` folder. Use `helpers/` for automation.
3. **Defensive Pointer Gates:** Always guard native C++ UObject calls with `pcall` and validity checks. Never assume an object pointer remains valid across frames.
4. **Zero Slate Scraping Invariant:** Never reflect transient Slate/UMG widgets across frames to deduce state. Query stable manager, actor, or controller properties on the `GameThread`.
