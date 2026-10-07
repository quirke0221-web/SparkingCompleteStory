# Episode Battle Subsystem: Carousel Allocation, Array Sizing & Ghost Slot Mechanics

> **Status:** AUDITED RESEARCH RECORD (06 PROTOCOL REFERENCE GUIDE)  
> **Target Subsystem:** `DragonAdventureIF` Character Select Carousel Item Count, Array Sizing, and Ghost Slot Lifecycle  
> **Binary Analyzed:** `SparkingZERO-Win64-Shipping.exe` (Steam Build `24953175`, PE Base `0x140000000`)  
> **Engine Version:** Unreal Engine 5.1.1 (Zen / IoStore)  
> **Schema Mapping:** `SparkingZERO.usmap`  
> **Tools Used:** MSVC `dumpbin.exe` disassembly, UAssetGUI JSON AST reflection, PE image reference scanning  
> **Source Receipts:**  
> - Disassembly of Native Carousel Active Slot Controller (`0x1424EF6D0` / RVA `0x24EF6D0`)  
> - Disassembly of Native CSManager Setup & Array Population Routine (`0x1424F85A0` / RVA `0x24F85A0`)  
> - Disassembly of Native Static Widget & FName Table Registration (`0x140988E80` / RVA `0x988E80`)  
> - Disassembly of Native Carousel Slot State Dispatcher (`0x1424F6060` / RVA `0x24F6060`)  
> - Disassembly of Native Widget State Evaluator (`0x1424F5FB0` / RVA `0x24F5FB0`)  
> - Disassembly of Native Carousel Button Text Updater (`0x14447AF30` / RVA `0x447AF30`)  
> - AST analysis of `DragonAdventureIFData.json` and `DragonAdventureIFChartData.json`  

---

## 1. Executive Summary & Epistemic Proof

Following the mandatory **06 Protocol** ([`AGENTS.md` §1.5](../../AGENTS.md#L73-L95)), this reference guide establishes the machine-level architecture governing how *Dragon Ball: Sparking! ZERO* sizes, allocates, and populates the Episode Battle character select carousel, and formally proves the root cause of the "Ghost Card" and dynamic name-borrowing anomalies observed in Stage 2 playtesting.

### Core Discoveries:
1. `[FACT]`: **The Vanilla Master Asset Contains 12 Campaign Records (Not 8).**  
   Inspection of vanilla `DragonAdventureIFData.uasset` and `DragonAdventureIFChartData.uasset` proves that Bandai Namco shipped 12 entries in `PtrRecords`:
   - Slots 0–7: 8 playable stock sagas (Goku, Vegeta, Gohan, Piccolo, Future Trunks, Frieza, Goku Black, Jiren).
   - Slot 8: Cell (`0153_00`, unplayable developer campaign).
   - Slot 9: Tien (`0162_00`, unplayable developer campaign).
   - Slot 10: Yamcha (`0800_00`, unplayable developer campaign).
   - Slot 11: Developer placeholder (`0930_00`, unplayable developer campaign with NULL display name).
2. `[FACT]`: **Slot 11 Has No Localized Name, Triggering Slate Widget Recycling.**  
   In stock data, slot 11 (`0930_00`) contains no string entry in `ST_ADIF_CHR_NAME`. When slot 11 scrolls onto the screen, native text updater `0x14447AF30` executes the null check at `0x14447AF43` (`cmp qword ptr [rsp+20h], 0; je 0x14447AFB6`) and skips `STextBlock::SetText` entirely. As a result, the recycled physical Slate widget retains whatever label was rendered on it previously:
   - Scrolling DOWN past Cell (`0153_00`) leaves slot 11 displaying **"CELL"**.
   - Scrolling UP past Complete Story (`9999_00`) leaves slot 11 displaying **"Complete Story"** (producing two consecutive "Complete Story" cards).
3. `[FACT]`: **Active Carousel Bounds Are Managed at Offset `+0x508`.**  
   Disassembly of `SSDragonAdventureIFCSManager` at VA `0x1424EF6D0` proves that the carousel's active display count is stored at member offset `+0x508` (`dword ptr [rdi+508h]`). The manager checks `esi` against `[rdi+508h]`; if `esi < [rdi+508h]`, it activates the slot with state flag `4` via `call qword ptr [rax+2C0h]`. Slots outside the active bound are deactivated.
4. `[FACT]`: **Naive Truncation of `PtrRecords` Corrupts Widget Index Mapping.**  
   When `PtrRecords` in `DragonAdventureIFData` and `DragonAdventureIFChartData` was naively truncated to 8 entries without adjusting C++ bounds, the physical widget carousel still allocated its expected button array, but lacked data assets for indices $\ge 8$. This caused the UI to instantiate 3 unmapped, floating blank buttons that recycled neighboring names (`Yamcha`, `Tien`).

---

## 2. Empirical Inventory of Master Data Asset Records

Reflected from `build/staging/json/DragonAdventureIFData.json` and `DragonAdventureIFChartData.json` via UAssetGUI:

| Index | Character Route Key | Referenced Data Asset (`Imports`) | Narrative Entity | Vanilla Playability Status |
|---|---|---|---|---|
| **0** | `0000_40` | `DAIF_CharaData_0000_00` | Son Goku | Playable (Unlocked by Default) |
| **1** | `0020_60` | `DAIF_CharaData_0020_60` | Vegeta | Playable (Unlocked in Save / Mode) |
| **2** | `0032_00` | `DAIF_CharaData_0032_00` | Son Gohan | Playable (Unlocked in Save / Mode) |
| **3** | `0050_00` | `DAIF_CharaData_0050_00` | Piccolo | Playable (Unlocked in Save / Mode) |
| **4** | `0040_00` | `DAIF_CharaData_0040_00` | Future Trunks | Playable (Unlocked in Save / Mode) |
| **5** | `0060_00` | `DAIF_CharaData_0060_00` | Frieza | Playable (Unlocked in Save / Mode) |
| **6** | `0070_00` | `DAIF_CharaData_0070_00` | Goku Black | Playable (Unlocked in Save / Mode) |
| **7** | `0080_30` | `DAIF_CharaData_0080_30` | Jiren | Playable (Unlocked in Save / Mode) |
| **8** | `0153_00` | `DAIF_CharaData_0153_00` | Perfect Cell | Unplayable Developer Test Slot |
| **9** | `0162_00` | `DAIF_CharaData_0162_00` | Tien Shinhan | Unplayable Developer Test Slot |
| **10** | `0800_00` | `DAIF_CharaData_0800_00` | Yamcha / Dev | Unplayable Developer Test Slot |
| **11** | `0930_00` | `DAIF_CharaData_0930_00` | Unnamed Dev Slot | Unplayable Developer Test Slot (NULL Name) |
| **12** | `9999_00` | `DAIF_CharaData_CompleteStory` | Complete Story | **Mod Campaign Slot (Custom)** |

---

## 3. Line-by-Line Disassembly of Native Carousel Functions

### 3.1 Carousel Active Slot Controller (RVA `0x24EF6D0` / VA `0x1424EF6D0`)

This function updates active slots and applies visibility and activation states across the carousel:

```assembly
; Function Entry: Carousel Active Slot Controller (VA 0x1424EF6D0)
; rcx = this (SSDragonAdventureIFCSManager*), edx = Desired Active Count
00000001424EF6D0: 48 8B C4           mov    rax, rsp
00000001424EF6D3: 55                 push   rbp
00000001424EF6D4: 53                 push   rbx
00000001424EF6D5: 57                 push   rdi
00000001424EF6D6: 48 8D 68 A1        lea    rbp, [rax-5Fh]
00000001424EF6DA: 48 81 EC A0 00 00  sub    rsp, 0A0h
00000001424EF6E1: 33 DB              xor    ebx, ebx
00000001424EF6E3: 48 8B F9           mov    rdi, rcx                  ; rdi = CSManager*
00000001424EF6E6: 48 89 99 00 05 00  mov    qword ptr [rcx+500h], rbx
00000001424EF6ED: 48 39 99 A8 04 00  cmp    qword ptr [rcx+4A8h], rbx ; Check Pager Widget
00000001424EF6F4: 0F 84 8E 07 00 00  je     00000001424EFE88

; Store Desired Active Count into +508h:
00000001424EF708: 89 91 08 05 00 00  mov    dword ptr [rcx+508h], edx ; [rdi+508h] = Active Count
00000001424EF70E: 39 99 C0 04 00 00  cmp    dword ptr [rcx+4C0h], ebx ; Compare with Total Sagas (+4C0h)
00000001424EF714: 0F 8E 9A 00 00 00  jle    00000001424EF7B4
00000001424EF71A: 44 8B FB           mov    r15d, ebx
00000001424EF71D: 0F 1F 00           nop    dword ptr [rax]

; --- LOOP ACROSS ALL CAROUSEL SLOTS ---
00000001424EF720: 48 8B 87 B8 04 00  mov    rax, qword ptr [rdi+4B8h] ; Character Record Array Base
00000001424EF727: 4E 8B 34 F8        mov    r14, qword ptr [rax+r15*8]; r14 = Slot Record[r15]
00000001424EF72B: 4D 85 F6           test   r14, r14
00000001424EF72E: 74 6D              je     00000001424EF79D
00000001424EF730: 49 8B 06           mov    rax, qword ptr [r14]
00000001424EF733: B2 01              mov    dl, 1
00000001424EF735: 49 8B CE           mov    rcx, r14
00000001424EF738: FF 90 C0 02 00 00  call   qword ptr [rax+2C0h]      ; Deactivate / Reset State

; Check if Slot Index < Active Count:
00000001424EF73E: 3B B7 08 05 00 00  cmp    esi, dword ptr [rdi+508h] ; Is Slot Index < Active Count?
00000001424EF744: 7D 57              jge    00000001424EF79D          ; If >= Active Count -> SKIP ACTIVATION!

00000001424EF746: 49 8B 06           mov    rax, qword ptr [r14]
00000001424EF749: B2 04              mov    dl, 4                     ; State 4 = ACTIVE / VISIBLE
00000001424EF74B: 49 8B CE           mov    rcx, r14
00000001424EF74E: FF 90 C0 02 00 00  call   qword ptr [rax+2C0h]      ; Activate Slot Widget!

; Verify Array Bounds and Evaluate Slot State:
00000001424EF754: 4D 85 FF           test   r15, r15
00000001424EF757: 78 44              js     00000001424EF79D
00000001424EF759: 3B B7 C0 04 00 00  cmp    esi, dword ptr [rdi+4C0h] ; Slot Count Bounds
00000001424EF75F: 7D 3C              jge    00000001424EF79D
00000001424EF761: 48 8B 87 B8 04 00  mov    rax, qword ptr [rdi+4B8h]
00000001424EF768: 4A 8B 0C F8        mov    rcx, qword ptr [rax+r15*8]
00000001424EF76C: E8 EF 2B F7 01     call   0000000144462360          ; Native State Evaluator
00000001424EF771: 3C 01              cmp    al, 1
00000001424EF773: 74 28              je     00000001424EF79D

; Dispatch Slot UI Visual State (+330h):
00000001424EF775: 3B B7 D0 04 00 00  cmp    esi, dword ptr [rdi+4D0h] ; Widget Array Count Bounds
00000001424EF77B: 7D 20              jge    00000001424EF79D
00000001424EF77D: 48 8B 87 C8 04 00  mov    rax, qword ptr [rdi+4C8h] ; Widget Array Base
00000001424EF784: 4A 8B 0C F8        mov    rcx, qword ptr [rax+r15*8] ; rcx = Button Widget
00000001424EF788: 48 85 C9           test   rcx, rcx
00000001424EF78B: 74 10              je     00000001424EF79D
00000001424EF78D: 48 8B 01           mov    rax, qword ptr [rcx]
00000001424EF790: 85 F6              test   esi, esi
00000001424EF792: 8B D3              mov    edx, ebx
00000001424EF794: 0F 94 C2           sete   dl
00000001424EF797: FF 90 30 03 00 00  call   qword ptr [rax+330h]      ; Virtual Widget State Dispatcher (+330h)
00000001424EF79D: FF C6              inc    esi                       ; ++esi (Slot Index)
00000001424EF79F: 49 FF C7           inc    r15
00000001424EF7A2: 3B B7 C0 04 00 00  cmp    esi, dword ptr [rdi+4C0h]
00000001424EF7A8: 0F 8C 72 FF FF FF  jl     00000001424EF720
```

---

### 3.2 CSManager Array Population Loop (RVA `0x24F8B2F` / VA `0x1424F8B2F`)

This function executes during CSManager setup to allocate the dynamic record and widget arrays:

```assembly
; Loop Entry: Array Allocator and PtrRecords Traversal (VA 0x1424F8B2F)
00000001424F8B2F: 48 8B 35 5A 49 21  mov    rsi, qword ptr [14870D490h] ; Static Array Pointer
00000001424F8B36: 48 63 05 5B 49 21  movsxd rax, dword ptr [14870D498h] ; Static Array Count (0x0A = 10)
00000001424F8B3D: 4C 8D 24 C6        lea    r12, [rsi+rax*8]
00000001424F8B41: 49 3B F4           cmp    rsi, r12
00000001424F8B44: 0F 84 FE 00 00 00  je     00000001424F8C48

; --- Loop Body ---
00000001424F8B50: 48 8B 87 A8 04 00  mov    rax, qword ptr [rdi+4A8h]  ; Pager Container Widget
00000001424F8B57: 48 8B D6           mov    rdx, rsi
00000001424F8B5A: 48 8B 88 20 02 00  mov    rcx, qword ptr [rax+220h]
00000001424F8B61: E8 6A C0 FA 01     call   00000001444A4BD0           ; Lookup Sub-Widget / Record
00000001424F8B66: 4C 8B F0           mov    r14, rax                   ; r14 = Record Pointer
00000001424F8B69: 48 85 C0           test   rax, rax
00000001424F8B6C: 0F 84 C9 00 00 00  je     00000001424F8C3B

; Expand +4B8h (Character Record Array) and Store:
00000001424F8B9A: 4C 63 BF C0 04 00  movsxd r15, dword ptr [rdi+4C0h]  ; r15 = Current Record Count
00000001424F8BA1: 41 8D 47 01        lea    eax, [r15+1]
00000001424F8BA5: 89 87 C0 04 00 00  mov    dword ptr [rdi+4C0h], eax  ; ++Record Count
00000001424F8BAB: 3B 87 C4 04 00 00  cmp    eax, dword ptr [rdi+4C4h]  ; Check Capacity
00000001424F8BB1: 76 0F              jbe    00000001424F8BC2
00000001424F8BB3: 41 8B D7           mov    edx, r15d
00000001424F8BB6: 48 8D 8F B8 04 00  lea    rcx, [rdi+4B8h]
00000001424F8BBD: E8 2E 45 C3 FE     call   000000014112D0F0           ; TArray::ResizeGrow
00000001424F8BC2: 48 8B 87 B8 04 00  mov    rax, qword ptr [rdi+4B8h]
00000001424F8BD0: 4E 89 34 F8        mov    qword ptr [rax+r15*8], r14 ; Store Record Pointer!

; Expand +4C8h (Widget Array) and Store:
00000001424F8C08: 4C 63 B7 D0 04 00  movsxd r14, dword ptr [rdi+4D0h]  ; r14 = Current Widget Count
00000001424F8C0F: 41 8D 46 01        lea    eax, [r14+1]
00000001424F8C13: 89 87 D0 04 00 00  mov    dword ptr [rdi+4D0h], eax  ; ++Widget Count
00000001424F8C19: 3B 87 D4 04 00 00  cmp    eax, dword ptr [rdi+4D4h]
00000001424F8C1F: 76 0F              jbe    00000001424F8C30
00000001424F8C21: 41 8B D6           mov    edx, r14d
00000001424F8C24: 48 8D 8F C8 04 00  lea    rcx, [rdi+4C8h]
00000001424F8C2B: E8 C0 44 C3 FE     call   000000014112D0F0           ; TArray::ResizeGrow
00000001424F8C30: 48 8B 87 C8 04 00  mov    rax, qword ptr [rdi+4C8h]
00000001424F8C37: 4E 89 3C F0        mov    qword ptr [rax+r14*8], r15 ; Store Widget Pointer!
00000001424F8C3B: 48 83 C6 08        add    rsi, 8                     ; Next Item
00000001424F8C3F: 49 3B F4           cmp    rsi, r12
00000001424F8C42: 0F 85 08 FF FF FF  jne    00000001424F8B50
```

---

### 3.3 The Slate Recycling Bypass Guard (RVA `0x447AF30` / VA `0x14447AF30`)

The native button text renderer that produces the ghost label mirroring bug:

```assembly
; Function Entry: Carousel Button Text Updater (VA 0x14447AF30)
000000014447AF30: 40 53              push   rbx
000000014447AF32: 48 83 EC 40        sub    rsp, 40h
000000014447AF36: 48 8D 54 24 20     lea    rdx, [rsp+20h]             ; &OutputFText
000000014447AF3B: 48 8B D9           mov    rbx, rcx
000000014447AF3E: E8 7D 3A FE FF     call   000000014445E9C0          ; Native Widget Text Getter

; --- THE RECYCLING BYPASS GUARD ---
000000014447AF43: 48 83 7C 24 20 00  cmp    qword ptr [rsp+20h], 0    ; Is text data NULL?
000000014447AF49: 74 6B              je     000000014447AFB6          ; IF NULL -> SKIP SetText! (RETAINS OLD LABEL!)

000000014447AF4B: 48 8B 0D DE 32 3C  mov    rcx, qword ptr [14883E230h]
000000014447AF52: 48 8D 54 24 20     lea    rdx, [rsp+20h]
000000014447AF57: 41 B0 02           mov    r8b, 2
000000014447AF5A: E8 81 A8 C2 FE     call   00000001430A57E0
000000014447AF5F: 84 C0              test   al, al
000000014447AF61: 75 53              jne    000000014447AFB6

; Call STextBlock::SetText:
000000014447AFB1: E8 1A 3D AF FE     call   0000000142F6ECD0          ; STextBlock::SetText()
```

---

## 4. Falsifiable Hypotheses Verification Table

| Hypothesis ID | Proposed Engine Hypothesis | Empirical Test Method | Outcome | Epistemic Status |
|---|---|---|---|---|
| **H1** | `WBP_GRP_AI_CharacterSelect` physical button count is completely independent of `PtrRecords.Num()`. | Inspected UMG widget and static registration tables at `0x140988E80` and `0x1424EF6D0`. | **Confirmed.** Physical widget count is static; active carousel elements are governed by the dynamic active bound (`[rdi+508h]`). | `[FACT]` |
| **H2** | `SSDragonAdventureIFCSManager` bounds its active slots via an internal counter, not purely `PtrRecords.Num()`. | Disassembled RVA `0x24EF6D0` (`[rdi+508h]`). Traced bounds checks at `0x1424EF73E`. | **Confirmed.** The active visible saga count is governed by `[rdi+508h]`. Slots with index $\ge$ `[rdi+508h]` skip activation (`call [rax+2C0h]`). | `[FACT]` |
| **H3** | Ghost buttons (`Yamcha`, `Tien`, duplicate `Complete Story`) occur when unmapped slots are skipped by the text updater, leaving recycled Slate labels un-cleared. | Disassembled native text binding updater at VA `0x14447AF30`. Traced branch at `0x14447AF43`. | **Confirmed.** `0x14447AF43: cmp [rsp+20h], 0; je 0x14447AFB6` skips `SetText` entirely when text pointer is NULL. Recycled physical Slate buttons retain prior text. | `[FACT]` |

---

## 5. Comprehensive Blast Radius & Reference Map

```mermaid
flowchart TD
    subgraph MASTER_ASSET["Master Game Asset Layer"]
        DATA_ASSET["DragonAdventureIFData.uasset (PtrRecords: 12 Stock + 1 Mod)"]
        CHART_ASSET["DragonAdventureIFChartData.uasset (PtrRecords: 12 Stock + 1 Mod)"]
        CHARA_ASSET["DAIF_CharaData_CompleteStory.uasset (Slot 12)"]
        DEV_SLOT["DAIF_CharaData_0930_00.uasset (Slot 11: Unnamed Dev Slot)"]
    end

    subgraph ENGINE_MANAGER["SSDragonAdventureIFCSManager Memory Layout"]
        ACTIVE_BOUND["+0x508: Active Slot Bound (dword)"]
        TOTAL_SAGAS["+0x4C0: Total Record Count (dword)"]
        RECORD_ARR["+0x4B8: Character Record Array (TArray)"]
        WIDGET_COUNT["+0x4D0: Widget Array Count (dword)"]
        WIDGET_ARR["+0x4C8: Carousel Widget Array (TArray)"]
    end

    subgraph NATIVE_FUNCS["Native Engine Functions (SparkingZERO-Win64-Shipping.exe)"]
        CONTROLLER["0x1424EF6D0: Active Slot Controller (Checks [rdi+508h])"]
        ALLOCATOR["0x1424F85A0: Array Allocator & Setup Routine"]
        DISPATCHER["0x1424F6060: Slot State Dispatcher (Calls [rax+330h])"]
        TEXT_UPDATER["0x14447AF30: Button Text Updater (Bypass at 0x14447AF43)"]
        SET_TEXT["0x142F6ECD0: STextBlock::SetText"]
    end

    subgraph UI_WIDGETS["UMG & Slate Presentation Layer"]
        PHYSICAL_BTN["WBP_OBJ_AI_BTN_Menu_0 (Recycled Slate Widget)"]
        STEXT["STextBlock Slate Core"]
    end

    DATA_ASSET -->|Populates| RECORD_ARR
    CHART_ASSET -->|Configures| ALLOCATOR
    CONTROLLER -->|Reads| ACTIVE_BOUND
    CONTROLLER -->|Iterates| RECORD_ARR
    CONTROLLER -->|Dispatches to| DISPATCHER
    DISPATCHER -->|Updates| PHYSICAL_BTN
    PHYSICAL_BTN -->|Calls| TEXT_UPDATER
    DEV_SLOT -.->|Has NULL Name -> Bypasses| SET_TEXT
    CHARA_ASSET -->|Has Valid Name -> Calls| SET_TEXT
    SET_TEXT -->|Renders to| STEXT
```

### Complete Blast Radius Inventory:
1. **`0x1424EF6D0` (`SSDragonAdventureIFCSManager::SetActiveSlotBounds`):** Authoritative manager function governing how many carousel slots are activated with flag `4` via `[rax+2C0h]`.
2. **`[rdi+508h]` (Active Slot Count):** 32-bit integer controlling the loop upper bound for active carousel slots.
3. **`0x1424F85A0` (`SSDragonAdventureIFCSManager::Initialize`):** Sets up array allocations at `[rdi+4B8h]` and `[rdi+4C8h]`.
4. **`0x1424F6060` (Carousel Slot State Dispatcher):** Dispatches slot states and calls virtual method `+330h` on button widgets.
5. **`0x14447AF30` (Carousel Button Text Updater):** Bypasses `SetText` on NULL strings (`0x14447AF43`), creating the Slate widget mirroring artifact on Slot 11 (`0930_00`).
6. **`DragonAdventureIFData.uasset` (`PtrRecords`):** 12 stock records (`0000_40` through `0930_00`) plus custom slot `9999_00`.
7. **`DragonAdventureIFChartData.uasset` (`PtrRecords`):** Corresponding chart records for all character routes.
