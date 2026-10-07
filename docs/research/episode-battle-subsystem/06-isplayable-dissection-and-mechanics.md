# Episode Battle Subsystem: `IsPlayable` Deep Dissection & Engine Mechanics

> **Status:** AUDITED RESEARCH RECORD  
> **Target Subsystem:** `DragonAdventureIF` Playability, Save Lookup & Carousel UI Construction  
> **Binary Analyzed:** `SparkingZERO-Win64-Shipping.exe` (Steam Build `24953175`, PE Base `0x140000000`)  
> **Tools Used:** MSVC `dumpbin.exe` disassembly, memory pattern analysis, live runtime telemetry (`CompleteStoryRuntime.log`)  
> **Source Receipts:**  
> - Disassembly of RVA `0x1EC35B0` (`execIsPlayable`)  
> - Disassembly of RVA `0x24F1530` (`SSDragonAdventureIFCSManager::IsPlayable`)  
> - Disassembly of RVA `0x2510ED0` (`IsCharacterPlayableInSave`)  
> - Disassembly of RVA `0x24F602D` (`SSDragonAdventureIFCSManager::UpdateCarouselWidgetState`)  

---

## 1. Executive Summary & Epistemic Proof

In earlier development milestones, our attempts to unlock the custom campaign tile resulted in a persistent paradox:
* **The Paradox:** Hooking `execIsPlayable` (RVA `0x1EC35B0`) successfully forced the function to return `true` (verified in `CompleteStoryRuntime.log`), and setting `DefaultOpenCharacter = "0000_00"` pointed the camera at our custom saga on startup. **Yet the tile remained locked, stubbornly displaying the orange "Unlock" button instead of the gold "New Game" badge.**
* **The Root Cause:** Through full binary disassembly of `SparkingZERO-Win64-Shipping.exe`, we proved that "IsPlayable" is not a single function, but a **three-tier execution hierarchy**. Crucially, the **UI Carousel Widget Builder** that decides whether a button is orange ("Unlock") or gold ("New Game") **completely bypasses the Blueprint VM thunk (`execIsPlayable`)** and calls the internal C++ save query engine directly.

```
┌────────────────────────────────────────────────────────────────────────┐
│ [Tier 1: Blueprint VM Execution Thunk]                                 │
│ RVA 0x1EC35B0: execIsPlayable (UFunction::Func)                        │
│ - Only called when Blueprint scripts invoke IsPlayable on click        │
└──────────────────────────────────┬─────────────────────────────────────┘
                                   │
                                   ▼
┌────────────────────────────────────────────────────────────────────────┐
│ [Tier 2: C++ Manager Member Function]                                  │
│ RVA 0x24F1530: SSDragonAdventureIFCSManager::IsPlayable               │
│ - Extracts CharacterKey (FKoratCharacterDataList) at offset +0x14B0    │
└──────────────────────────────────┬─────────────────────────────────────┘
                                   │ (JMP)
                                   ▼
┌────────────────────────────────────────────────────────────────────────┐
│ [Tier 3: Core Save Data Query Engine]                                  │
│ RVA 0x2510ED0: IsCharacterPlayableInSave(FKoratCharacterDataList* Key) │
│ - Performs TMap::Find on CharacterPlayableData                         │
│ - Checks if UnlockInfo != 0                                            │
└──────────────────────────────────▲─────────────────────────────────────┘
                                   │
   Direct C++ Call (Bypasses Tier 1 & 2!)
                                   │
┌──────────────────────────────────┴─────────────────────────────────────┐
│ [UI Carousel Widget Builder]                                           │
│ RVA 0x24F602D: SSDragonAdventureIFCSManager::UpdateCarouselPanel       │
│ - Evaluates button appearance when menu items are constructed          │
│ - Returns edx = 0 ("Unlock" button) or edx = 1 ("New Game" badge)      │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Complete Disassembly of the 3-Tier Hierarchy

### Tier 1: `execIsPlayable` (RVA `0x1EC35B0`, File Offset `0x1EC2BB0`)

This is the native C++ thunk registered in `UFunction::Func` (offset `0xD8`) that Unreal Engine's virtual machine calls when Blueprint bytecode executes the `IsPlayable` node:

```assembly
; Calling convention: void execIsPlayable(UObject* Context, FFrame& Stack, void* RESULT_DECL)
; rcx = Context (SSDragonAdventureIFCSManager*)
; rdx = Stack (FFrame&)
; r8  = RESULT_DECL (bool* ReturnValue buffer)

0000000141EC35B0: 40 53              push   rbx
0000000141EC35B2: 48 83 EC 20        sub    rsp, 20h
0000000141EC35B6: 48 8B 42 20        mov    rax, qword ptr [rdx+20h] ; rax = Stack.Code
0000000141EC35BA: 45 33 C9           xor    r9d, r9d
0000000141EC35BD: 48 85 C0           test   rax, rax
0000000141EC35C0: 49 8B D8           mov    rbx, r8                  ; rbx = RESULT_DECL
0000000141EC35C3: 41 0F 95 C1        setne  r9b
0000000141EC35C7: 4C 03 C8           add    r9, rax
0000000141EC35CA: 4C 89 4A 20        mov    qword ptr [rdx+20h], r9
0000000141EC35CE: E8 5D DF 62 00     call   00000001424F1530         ; Call Tier 2 (Manager::IsPlayable)
0000000141EC35D3: 88 03              mov    byte ptr [rbx], al       ; Store returned AL (bool) into [RESULT_DECL]
0000000141EC35D5: 48 83 C4 20        add    rsp, 20h
0000000141EC35D9: 5B                 pop    rbx
0000000141EC35DA: C3                 ret
```

* `[FACT]`: In our previous implementation, we hooked this function. When invoked, it successfully wrote `1` to `[rbx]`. However, this function is **only executed when clicking/confirming** in Blueprint, never when building the carousel widgets.

---

### Tier 2: `SSDragonAdventureIFCSManager::IsPlayable` (RVA `0x24F1530`, File Offset `0x24F0B30`)

This is the compiled C++ implementation method of `SSDragonAdventureIFCSManager`:

```assembly
; Calling convention: bool SSDragonAdventureIFCSManager::IsPlayable(this)
; rcx = this (SSDragonAdventureIFCSManager*)

00000001424F1530: 40 53              push   rbx
00000001424F1532: 48 83 EC 20        sub    rsp, 20h
00000001424F1536: E8 05 F9 F6 01     call   0000000144460E40         ; Get Current Character Context Object
00000001424F153B: 48 8B D8           mov    rbx, rax
00000001424F153E: 48 85 C0           test   rax, rax
00000001424F1541: 74 31              je     00000001424F1574         ; If null, return false (al = 0)
00000001424F1543: E8 58 49 A3 FF     call   0000000141F25EA0
00000001424F1548: 48 8B 4B 10        mov    rcx, qword ptr [rbx+10h]
00000001424F154C: 48 83 C0 30        add    rax, 30h
00000001424F1550: 48 63 50 08        movsxd rdx, dword ptr [rax+8]
00000001424F1554: 3B 51 38           cmp    edx, dword ptr [rcx+38h]
00000001424F1557: 7F 1B              jg     00000001424F1574
00000001424F1559: 48 8B 49 30        mov    rcx, qword ptr [rcx+30h]
00000001424F155D: 48 39 04 D1        cmp    qword ptr [rcx+rdx*8], rax
00000001424F1561: 75 11              jne    00000001424F1574
00000001424F1563: 48 8D 8B B0 14 00  lea    rcx, [rbx+14B0h]         ; rcx = &Character->CharacterKey
00000001424F156A: 48 83 C4 20        add    rsp, 20h
00000001424F156E: 5B                 pop    rbx
00000001424F156F: E9 5C F9 01 00     jmp    0000000142510ED0         ; TAIL-CALL: JMP Tier 3!
00000001424F1574: 32 C0              xor    al, al                   ; Return false
00000001424F1576: 48 83 C4 20        add    rsp, 20h
00000001424F157A: 5B                 pop    rbx
00000001424F157B: C3                 ret
```

* `[FACT]`: Tier 2 does not contain any business logic of its own. It simply retrieves the character key struct `FKoratCharacterDataList` at offset `+0x14B0` and tail-calls directly into Tier 3 (`0x142510ED0`).

---

### Tier 3: `IsCharacterPlayableInSave` (RVA `0x2510ED0`, File Offset `0x25104D0`)

This is the authoritative, standalone C++ engine function that inspects the persistent save data for any given character key:

```assembly
; Calling convention: bool IsCharacterPlayableInSave(const FKoratCharacterDataList* Key)
; rcx = Key pointer (points to 8-byte FName: [ComparisonIndex: u32, Number: u32])

0000000142510ED0: 48 89 5C 24 08     mov    qword ptr [rsp+8], rbx
0000000142510ED5: 57                 push   rdi
0000000142510ED6: 48 83 EC 20        sub    rsp, 20h
0000000142510EDA: 48 8B F9           mov    rdi, rcx                 ; rdi = Key pointer
0000000142510EDD: E8 FE E9 F4 FF     call   000000014245F8E0         ; Get USSSaveSubsystem
0000000142510EE2: 48 85 C0           test   rax, rax
0000000142510EE5: 0F 84 94 00 00 00  je     0000000142510F7F         ; If null, return false
0000000142510EEB: 48 8B 98 30 09 00  mov    rbx, qword ptr [rax+930h]; rbx = SaveSubsystem->MainGameSaveData
0000000142510EF2: 48 85 DB           test   rbx, rbx
0000000142510EF5: 0F 84 84 00 00 00  je     0000000142510F7F         ; If null, return false
0000000142510EFB: 48 8B 9B 80 00 00  mov    rbx, qword ptr [rbx+80h] ; rbx = &DragonAdventureIFSaveData.CharacterPlayableData (TMap)
0000000142510F02: 48 85 DB           test   rbx, rbx
0000000142510F05: 0F 84 74 00 00 00  je     0000000142510F7F         ; If null, return false
0000000142510F0B: 8B 83 98 0C 00 00  mov    eax, dword ptr [rbx+0C98h] ; TMap Element Count
0000000142510F11: 3B 83 C4 0C 00 00  cmp    eax, dword ptr [rbx+0CC4h]
0000000142510F17: 74 66              je     0000000142510F7F         ; If TMap is empty, return false

; --- TMap Hash Calculation & Bucket Lookup ---
0000000142510F19: 48 8B 07           mov    rax, qword ptr [rdi]     ; Load 8-byte FName Key
0000000142510F1C: 8B C8              mov    ecx, eax                 ; ecx = FName.ComparisonIndex
0000000142510F1E: 48 89 44 24 38     mov    qword ptr [rsp+38h], rax
0000000142510F23: E8 18 51 7F 00     call   0000000142D06040         ; GetTypeHash(FName)
0000000142510F28: 03 44 24 3C        add    eax, dword ptr [rsp+3Ch] ; Add FName.Number to hash
0000000142510F2C: 4C 8D 83 C8 0C 00  lea    r8, [rbx+0CC8h]          ; r8 = TMap Hash Buckets base
0000000142510F33: 49 8B 48 08        mov    rcx, qword ptr [r8+8]
0000000142510F37: 48 63 93 D8 0C 00  movsxd rdx, dword ptr [rbx+0CD8h] ; rdx = Number of Buckets
0000000142510F3E: 48 FF CA           dec    rdx                      ; rdx = Mask (Buckets - 1)
0000000142510F41: 48 98              cdqe
0000000142510F43: 48 23 D0           and    rdx, rax                 ; BucketIndex = Hash & Mask
0000000142510F46: 48 85 C9           test   rcx, rcx
0000000142510F49: 4C 0F 45 C1        cmovne r8, rcx
0000000142510F4D: 41 8B 04 90        mov    eax, dword ptr [r8+rdx*4] ; eax = Bucket Head Element Index
0000000142510F51: 83 F8 FF           cmp    eax, 0FFFFFFFFh          ; Is Bucket empty (-1)?
0000000142510F54: 74 29              je     0000000142510F7F         ; KEY NOT FOUND -> Return false (0)

; --- TMap Bucket Collision Chain Walk ---
0000000142510F56: 4C 8B 83 90 0C 00  mov    r8, qword ptr [rbx+0C90h] ; r8 = Element Array base
0000000142510F5D: 48 8B 0F           mov    rcx, qword ptr [rdi]     ; rcx = Target Key FName
0000000142510F60: 48 63 D0           movsxd rdx, eax
0000000142510F63: 48 8D 04 92        lea    rax, [rdx+rdx*4]
0000000142510F67: 48 8D 14 85 00 00  lea    rdx, [rax*4]
0000000142510F6F: 4A 39 0C 02        cmp    qword ptr [rdx+r8], rcx  ; Compare Element Key with Target Key
0000000142510F73: 74 17              je     0000000142510F8C         ; MATCH FOUND! Jump to Value evaluation!
0000000142510F75: 42 8B 44 02 0C     mov    eax, dword ptr [rdx+r8+0Ch] ; Load NextElementIndex in chain
0000000142510F7A: 83 F8 FF           cmp    eax, 0FFFFFFFFh
0000000142510F7D: 75 E1              jne    0000000142510F60         ; Loop next element in chain

; --- KEY NOT FOUND EXIT ---
0000000142510F7F: 32 C0              xor    al, al                   ; al = 0 (false)
0000000142510F81: 48 8B 5C 24 30     mov    rbx, qword ptr [rsp+30h]
0000000142510F86: 48 83 C4 20        add    rsp, 20h
0000000142510F8A: 5F                 pop    rdi
0000000142510F8B: C3                 ret                             ; Return false

; --- KEY MATCHED: EVALUATE VALUE (FSSDragonAdventureIFCharacterPlayableSaveData) ---
0000000142510F8C: 4A 8D 0C 02        lea    rcx, [rdx+r8]            ; rcx = &TMapElement
0000000142510F90: 33 D2              xor    edx, edx                 ; edx = 0
0000000142510F92: 48 85 C9           test   rcx, rcx
0000000142510F95: 48 8D 41 08        lea    rax, [rcx+8]             ; rax = Value struct (offset +8)
0000000142510F99: 48 0F 44 C2        cmove  rax, rdx
0000000142510F9D: 48 85 C0           test   rax, rax
0000000142510FA0: 74 DD              je     0000000142510F7F         ; If null, return false
0000000142510FA2: 38 10              cmp    byte ptr [rax], dl       ; COMPARE [rax] (UnlockInfo byte) with 0!
0000000142510FA4: 48 8B 5C 24 30     mov    rbx, qword ptr [rsp+30h]
0000000142510FA9: 0F 95 C0           setne  al                       ; al = (UnlockInfo != 0) !
0000000142510FAC: 48 83 C4 20        add    rsp, 20h
0000000142510FB0: 5F                 pop    rdi
0000000142510FB1: C3                 ret                             ; Return al (true if Unlocked, false if Locked)
```

### Key Discoveries in Tier 3:
1. `[FACT]`: The function takes a single argument in `rcx`: a pointer to `FKoratCharacterDataList` (which is an 8-byte `FName`).
2. `[FACT]`: It performs a full Unreal Engine `TMap::Find` operation directly against the live in-memory save data (`CharacterPlayableData`).
3. `[FACT]`: If the key is **not in the map**, it immediately returns `false` (`al = 0`).
4. `[FACT]`: If the key is in the map, it evaluates `UnlockInfo` at byte offset `+0` of the value struct.
   - If `UnlockInfo == 0` (`Lock`), it returns `false` (`0`).
   - If `UnlockInfo != 0` (`New` = 1 or `Checked` = 2), it returns `true` (`1`).

---

## 3. The 6 Callers of Tier 3 (`0x2510ED0`)

Our automated scan of the executable's `.text` section revealed that Tier 3 (`0x2510ED0`) is called from **6 distinct locations**:

| Caller RVA | Calling Type | Subsystem Component | Role in Episode Battle |
|---|---|---|---|
| **`0x24F602D`** | `CALL` | `SSDragonAdventureIFCSManager::UpdateCarouselPanel` | **The UI Widget Builder:** Sets the button to orange ("Unlock") or gold ("New Game")! |
| **`0x24F156F`** | `JMP` | `SSDragonAdventureIFCSManager::IsPlayable` | Tier 2 member function called by Tier 1 (`execIsPlayable`). |
| **`0x24FEA3F`** | `CALL` | `SSDragonAdventureIFCSManager::CheckChapterLaunch` | Validates whether chapter selection can proceed to combat. |
| **`0x2501F51`** | `CALL` | `SSDragonAdventureIFCSManager::EvaluateFlowchartNode` | Determines flowchart map line unlocks and what-if branching. |
| **`0x2513BA6`** | `CALL` | `SSDragonAdventureIFSaveData::UpdatePlayableState` | Updates mission completion flags in memory. |
| **`0x2513C06`** | `CALL` | `SSDragonAdventureIFSaveData::QueryCompletionStatus`| Validates whether character story clear achievements trigger. |

---

## 4. The Smoking Gun: The UI Widget Builder (RVA `0x24F602D`)

Here is the exact disassembly of caller #1 (RVA `0x24F602D`), which builds each carousel tile on screen:

```assembly
; Inside SSDragonAdventureIFCSManager::UpdateCarouselPanel:
00000001424F6026: 48 8D 8B B0 14 00  lea    rcx, [rbx+14B0h]         ; rcx = &Character->CharacterKey
00000001424F602D: E8 9E AE 01 00     call   0000000142510ED0         ; CALL Tier 3: IsCharacterPlayableInSave!
00000001424F6032: 84 C0              test   al, al                   ; Is character playable?
00000001424F6034: 74 07              je     00000001424F603D         ; If FALSE -> Jump to Locked!
00000001424F6036: BA 01 00 00 00     mov    edx, 1                   ; TRUE: edx = 1 (Gold "New Game" Badge)
00000001424F603B: EB 02              jmp    00000001424F603F
00000001424F603D: 33 D2              xor    edx, edx                 ; FALSE: edx = 0 (Orange "Unlock" Button)
00000001424F603F: 48 8B 8F A0 04 00  mov    rcx, qword ptr [rdi+4A0h] ; rcx = Carousel Widget Pointer
00000001424F6046: 48 8B 5C 24 30     mov    rbx, qword ptr [rsp+30h]
00000001424F604B: 48 83 C4 20        add    rsp, 20h
00000001424F604F: 5F                 pop    rdi
00000001424F6050: E9 9B B4 65 00     jmp    0000000142B514F0         ; TAIL-CALL: SetCarouselWidgetState(Widget, edx)!
```

### Why our earlier fix failed:
1. When you opened Episode Battle, the game constructed the carousel panels for all 13 sagas.
2. For tile 13 (`0000_00`), the engine called `UpdateCarouselPanel` (RVA `0x24F602D`).
3. It directly called Tier 3 (`0x2510ED0`). **It did not call `execIsPlayable`!**
4. Tier 3 searched `CharacterPlayableData` in your save file. Because `0000_00` was not found, it returned `al = 0`.
5. The UI builder set `edx = 0` and dispatched `SetCarouselWidgetState(Widget, 0)`.
6. This immediately rendered the orange **"Unlock"** button on screen!
7. `execIsPlayable` (where our MinHook detour was attached) was only called when you hovered over or pressed a button on the tile—long after the visual state had already been locked as orange.

---

## 5. Post-Mortem of Previous Hypotheses

| Prior Hypothesis | Tested Action | Empirical Outcome | Root Cause Disproved |
|---|---|---|---|
| **H1: Lua Map Mutation** | `cpd["0000_00"] = val_goku` in RE-UE4SS | `Tried setting member variable '0000_00' but UObject instance is nullptr` | RE-UE4SS Lua cannot allocate native Unreal `TMap` hash buckets or call `FMemory::Malloc`. |
| **H2: Asset Default Open** | `DefaultOpenCharacter = "0000_00"` in `DragonAdventureIFData.uasset` | Camera pointed at Goku (`0000_00`) on menu startup, but button remained orange "Unlock". | `DefaultOpenCharacter` only sets the initial carousel cursor index (`SelectedIndex`), not save unlock status. |
| **H3: Blueprint Thunk Hook** | MinHook detour on `execIsPlayable` (RVA `0x1EC35B0`) | Hook fired 6 times (`[EXEC_HOOK] Overrode execIsPlayable -> true`), but button remained orange "Unlock". | Carousel Widget Builder at RVA `0x24F602D` calls Tier 3 C++ function `0x2510ED0` directly, completely bypassing `execIsPlayable`. |

---

## 6. The Authoritative Native Resolution Plan

Because **all 6 systems** in the game (the UI widget builder, the click handler, chapter launch, and flowchart evaluation) converge on **Tier 3 (`0x2510ED0`)**, detouring Tier 3 solves every problem simultaneously:

### 6.1 Native Hook Specification
* **Target Address:** `Base + 0x2510ED0` (File offset `0x25104D0`)
* **Signature Prologue:** `48 89 5C 24 08 57 48 83 EC 20 48 8B F9` (`mov [rsp+8], rbx; push rdi; sub rsp, 20h; mov rdi, rcx`)
* **Calling Convention:** `bool detour_is_character_playable_in_save(const FName* key_ptr)`

### 6.2 Detour Logic
```rust
unsafe extern "C" fn detour_is_character_playable_in_save(key_ptr: *const FName) -> bool {
    if !key_ptr.is_null() {
        let key = *key_ptr;
        // If the queried character key matches our custom route key ("0000_00"):
        if key.comparison_index == CUSTOM_ROUTE_KEY_INDEX.load(Ordering::Relaxed) {
            return true; // Overrides UI Widget Builder, IsPlayable, and Chapter Launch!
        }
    }

    // Otherwise, call original native TMap lookup for stock vanilla characters
    let original: FnIsPlayableInSave = std::mem::transmute(ORIGINAL_IS_PLAYABLE_IN_SAVE.load(Ordering::Relaxed));
    original(key_ptr)
}
```

### 6.3 Player-Facing Outcomes:
1. **Gold "New Game" Badge:** When `UpdateCarouselPanel` calls `0x2510ED0`, it receives `al = 1`, sets `edx = 1`, and turns the badge gold immediately on menu construction.
2. **Correct Button Dispatch:** With `edx = 1`, `SSBuiltInMenu` binds `NewDecideButton` instead of `DecideButton`.
3. **Steam DLC Modal Bypassed:** Pressing Confirm triggers `IsModeStart()` and launches the story without querying the Steam DLC license system.
4. **Zero Save Corruption:** The player's disk save file (`MainGameSaveData.sav`) is never modified or contaminated.
