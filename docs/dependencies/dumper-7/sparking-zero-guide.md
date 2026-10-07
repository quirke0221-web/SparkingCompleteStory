# Dumper-7: Dragon Ball Sparking! ZERO Integration Guide

> **Status:** AUDITED REFERENCE  
> **Target Game:** Dragon Ball: Sparking! ZERO (Unreal Engine 5.1.1)  
> **Core Role:** Automated C++ SDK generation for memory layouts and function signatures  

---

## 1. Role in the Modding Harness

Dumper-7 is the foundational source of truth for **all memory layouts, UObject class definitions, struct padding, and native function signatures**.

* In shipping Unreal Engine binaries without PDB debug symbols, C++ classes are stripped of raw source code.
* However, Unreal Engine retains its full reflection system in memory (`GUObjectArray` and `GNames`).
* Dumper-7 traverses this reflection system directly from live RAM and emits clean, compilable C++ header files (`.hpp`).

---

## 2. Key Sparking! ZERO Classes Mapped by Dumper-7

When dumped for Sparking! ZERO, Dumper-7 outputs headers into `.tools/SDK/` (or `C:\Dumper-7\`):

1. **`ASSDragonAdventureIFCSManager.hpp`:**
   * Contains the exact member variables of the Episode Battle manager:
     - Carousel character selection pointers.
     - Playable character array at `+0x410` / length at `+0x418`.
   * Function signatures:
     - `bool IsPlayable();`
     - `bool IsModeStart();`
2. **`SSMainGameSaveData.hpp` / `SSSystemSaveData.hpp`:**
   * Contains exact memory offsets for `CharacterPlayableData` and save structures.
3. **`KoratCharacterDataList.hpp`:**
   * Defines the exact struct layout for character keys used by `DefaultOpenCharacter`.

---

## 3. Epistemic Invariants
* `[FACT]`: Dumper-7 handles bitfield alignment automatically, eliminating manual offset miscalculation.
* `[FACT]`: When writing native Rust or C++ plugins (`crates/complete-story-runtime`), consulting the generated SDK headers replaces all need for opcode guessing or pattern disassembly.
* `[POLICY]`: Generated SDK files (`.hpp`) belong in `.tools/SDK/` or documentation; never write native hooks without cross-referencing the SDK definition.
