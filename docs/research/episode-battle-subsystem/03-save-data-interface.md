# Episode Battle Subsystem: Save Data Persistence Interface

> **Status:** AUDITED RESEARCH RECORD  
> **Target Structs:** `SSDragonAdventureIFSaveData`, `CharacterPlayableData`, `EKoratUnLockMode`  
> **Source Receipts:**  
> - `evidence/object-dump/targeted-symbols.txt` (commit `e719f6b0`, lines 46361–72923)  
> - `evidence/runtime/v0.7-full-sanitized.log`  
> - `UE4SS.log` runtime error traces line 404  

---

## 1. Save Data Hierarchy & Subsystem Embedding

In *Dragon Ball: Sparking! ZERO*, story progression is stored inside the main game save file (`MainGameSaveData.sav`):

```text
/Script/SS.SSMainGameSaveData
  └── SSDragonAdventureIFSaveData (Property)
        └── CharacterPlayableData (MapProperty)
              ├── Key:   FKoratCharacterDataList (e.g. "0000_40")
              └── Value: FSSDragonAdventureIFCharacterPlayableSaveData
```

---

## 2. Reflected Struct & Enum Definitions

### 2.1 The `EKoratUnLockMode` Enumeration
From `targeted-symbols.txt` lines 46361–46365:

```cpp
enum class EKoratUnLockMode : uint8
{
    Lock        = 0, // Tile is locked; displays orange "Unlock" button
    New         = 1, // Tile is freshly unlocked; displays gold "New Game" badge!
    Checked     = 2, // Tile has been entered / story is in progress
    CheckedLock = 3  // Locked condition after specific route branch
};
```

* `[FACT]`: The gold **"New Game"** badge that we have been seeking is mathematically defined by `EKoratUnLockMode::New` (numeric value `1`).
* `[FACT]`: When a tile displays `Lock = 0` (or when the character key is absent from the map), the UI renders the orange "Unlock" button instead of the gold badge.

### 2.2 The `CharacterPlayableData` Map Structs
From `targeted-symbols.txt` lines 72869–72923:

```cpp
struct FSSDragonAdventureIFCharacterPlayableSaveData
{
    EKoratUnLockMode UnlockInfo; // Offset verified in reflection
    // Additional progression / route clear bitfields
};

// MapProperty inside SSDragonAdventureIFSaveData:
TMap<FKoratCharacterDataList, FSSDragonAdventureIFCharacterPlayableSaveData> CharacterPlayableData;
```

---

## 3. Why Lua Runtime Save Injection Failed (Post-Mortem)

In earlier iterations, we attempted to write `cpd["0000_00"] = val_goku` from Lua via RE-UE4SS. This failed consistently:

1. **Native Unreal `TMap` Memory Layout:**
   - Unreal Engine's `TMap` is an open-addressing hash table with an element array and hash buckets.
   - Inserting a new key into a `TMap` requires allocating memory via Unreal's native memory allocator (`FMemory::Malloc`) and rehashing the buckets.
2. **RE-UE4SS Lua Bridge Limitation:**
   - In RE-UE4SS `v3.0.1 Beta`, the Lua reflection wrapper supports reading existing keys from a `TMap`, but **cannot invoke the native C++ `TMap::Emplace` or `TMap::Add` allocator**.
   - Attempting to assign an unmapped key triggered: `[Lua][Error] Tried setting member variable '0000_00' but UObject instance is nullptr`.
3. **Engine Freezing & Lag:**
   - Because the write failed, the script repeatedly ran `FindAllOf("SSMainGameSaveData")` across frames, scanning 100,000+ UObjects in `GUObjectArray` on every carousel tick and stalling the game thread.

---

## 4. Non-Destructive Coexistence Invariant (`AGENTS.md` §4.3)

* `[POLICY]`: Any custom mod that touches save data must **NEVER corrupt or overwrite the player's vanilla save data**.
* If custom keys are injected into `MainGameSaveData.sav` and the player later uninstalls the mod, the vanilla game could fail to deserialize the unknown key `0000_00`, resulting in a corrupted save file error.
* **The Golden Path:**
  1. Primary unlock should be resolved at the **engine asset level** (`DefaultOpenCharacter`) or **native ABI execution level** (`execIsPlayable`).
  2. If save data injection is required, it must be transient in memory or scoped strictly to session initialization, never committed as corrupting disk serialization.

---

## 5. Epistemic Assessment

* `[FACT]`: The gold "New Game" badge is governed by `UnlockInfo == EKoratUnLockMode::New` (value `1`).
* `[FACT]`: `CharacterPlayableData` is an Unreal `TMap` that cannot be dynamically expanded from Lua script.
* `[OBSERVATION]`: On a completely fresh vanilla save, `CharacterPlayableData` has not yet recorded all characters; Goku (`0000_40`) is unlocked because the engine's default logic treats `DefaultOpenCharacter` as unlocked even when absent from save data.
