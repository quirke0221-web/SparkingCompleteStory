---
name: dumper-7-sdk-generator
description: Mandatory before writing native C++ or Rust memory hooks, reading game structs, or calculating function RVAs for Dragon Ball Sparking! ZERO. Governs Dumper-7 SDK generation, C++ struct alignments, bitfield decoding, and SDK header consumption.
---

# Skill: Dumper-7 SDK Generator

> **Status:** AUDITED & ACTIVE  
> **Pinned Tool:** Dumper-7 (x64 Unreal Engine SDK Generator)  
> **Target Game:** Dragon Ball: Sparking! ZERO (Unreal Engine 5.1.1)  
> **Reference folder:** [`docs/dependencies/dumper-7/`](../../../docs/dependencies/dumper-7/README.md)  
> (verbatim upstream receipts: `README.md`, `raw-readme.md`, `sparking-zero-guide.md`)

Every rule below cites its receipt. `[D7]` = upstream Dumper-7 documentation, `[SZ]` = `sparking-zero-guide.md`, `[LOCAL]` = local repository assets/configurations.

---

## 1. Scope Boundaries

**Use this skill for:**
- Generating strongly-typed C++ header files (`.hpp`) directly from live Unreal Engine 5.1 reflection memory (`GUObjectArray`, `GNames`).
- Resolving struct member variable offsets, byte sizes, and bitfield masks without manual reverse engineering or disassembly guessing.
- Verifying native `UFunction` signatures, execution thunk pointers, and parameter frame structures.
- Supplying authoritative struct layouts for native Rust plugins (`crates/complete-story-runtime`).

**Do NOT use this skill for (hand off instead):**
| Task | Owner skill |
|---|---|
| Attaching MinHook detours or installing native hooks in Rust | `minhook-native-hooking` |
| Lua-level reflection and UE4SS script hooks | `ue4ss-runtime-scripting` |
| Serializing binary assets to JSON or building packages | `uassetgui-asset-serialization` / `retoc-iostore-packer` |
| CLI pipeline build and packaging | `rust-pipeline-builder` |

---

## 2. Core Engine Invariants & SDK Generation

### 2.1 Live Reflection Inspection
- `[D7]` **Memory Introspection:** In shipping builds lacking PDB debug symbols, Dumper-7 traverses the live engine reflection system (`GUObjectArray`).
- `[SZ]` **SDK Output:** Emitted headers provide compilable C++ classes and struct declarations with exact padding offsets (e.g., `uint8 Pad_XXX[0xYY];`).
- `[FACT]` **Bitfield Handling:** Unreal Engine packs multiple boolean properties into bitfields (e.g. `uint8 bIsPlayable : 1`). Dumper-7 automatically calculates bit positions and masks, preventing manual offset misalignment.

### 2.2 Storage & File Invariants
- `[POLICY]` Generated SDK headers must be placed in `.tools/SDK/` or dedicated reference folders.
- `[POLICY]` Do not commit massive 500MB+ complete dumps into the git root. Archive only relevant class headers or inspect locally on demand.

---

## 3. Verified Golden Paths

### 3.1 Resolving Episode Battle Manager Layout
1. Generate SDK while game is running or consume cached `.tools/SDK/SparkingZERO/` headers.
2. Locate `SS_classes.hpp` or `ASSDragonAdventureIFCSManager.hpp`:
   - Inspect member variables at offsets:
     - `PtrRecords` array (`0x3F0` - `0x400`).
     - `PlayableCharacters` array (`0x410` - `0x420`).
     - `DefaultOpenCharacter` property offset.
   - Inspect function declarations:
     - `bool IsPlayable();`
     - `bool IsModeStart();`
3. Translate verified C++ offsets into Rust struct definitions in `crates/complete-story-runtime`.

### 3.2 Resolving Native `UFunction` Call Convention
- Native `UFUNCTION(BlueprintCallable)` functions store their machine code entry point at `UFunction::Func` (offset `0xD8`).
- The execution signature is always:
  ```cpp
  typedef void (*FNativeFuncPtr)(UObject* Context, FFrame* Stack, void* RESULT_DECL);
  ```
- Use Dumper-7's `FFrame` definition to read parameters via `Stack->StepCompiledIn()` or write return booleans directly to `*reinterpret_cast<bool*>(RESULT_DECL)`.
