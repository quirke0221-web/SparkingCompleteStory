# Option A: Runtime Memory Injection Plan (Path to Power Strategy)

**Objective:** Force the custom "Complete Story" Goku character tile (`0000_00`) to display the gold "New Game" badge and be playable by injecting its unlock status directly into the game's RAM, completely bypassing the native C++ function calls that blocked Option B.

**Technology Stack:** UE4SS (Lua Scripting) 3.0.1 Beta.

---

## Phase 1: Memory Discovery & Verification
Before we inject anything, we must prove we can see the game's live save data in RAM.

1. **Locate the Live Instance:** 
   - We will write a lightweight UE4SS Lua script that hooks a reliable early-game event (e.g., opening the Main Menu or Episode Battle screen).
   - Use this hook to capture the live instance of the central Manager or SaveGame object (e.g., `USSDragonAdventureIFManager` or `USSSaveData`).
2. **Read the TMap:**
   - Access the `CharacterPlayableData` `TMap` property on the live instance.
   - Iterate through it and print the contents to `UE4SS.log` to prove we can read the vanilla unlocked characters (like Goku, Vegeta, Piccolo).

## Phase 2: Transient Injection (The Core Fix)
Once we have access to the `TMap`, we inject our custom data.

1. **The Injection Hook:** 
   - We will hook the exact moment the player enters the Episode Battle menu (to ensure the Save Data is fully loaded but before the UI renders).
2. **GameThread Safety:**
   - Using UE4SS's `ExecuteInGameThread`, we will safely insert a new key-value pair into the `CharacterPlayableData` TMap:
     - **Key:** `0000_00` (Our custom Goku ID)
     - **Value:** `1` or `True` (Unlocked Status)
3. **Result:**
   - Because this happens *before* the C++ UI code checks the map, the native C++ code will look at the RAM, see that `0000_00` is unlocked, and natively render the gold "New Game" badge.

## Phase 3: The "Save Data" Safety Guard
We absolutely cannot risk corrupting the user's permanent `MainGameSaveData.sav` file with unofficial data. 

1. **Serialization Audit:**
   - We will trigger a manual save in the game (e.g., changing a setting) while our custom key is injected.
   - We will inspect the raw save file on disk to ensure `0000_00` was not permanently serialized.
2. **Failsafe Implementation (If needed):**
   - If the game attempts to save our custom key, we will implement a cleanup hook (e.g., hooking `USSSaveGame::SaveGameToSlot`) that temporarily deletes `0000_00` from RAM just before the save occurs, and re-injects it immediately after.

## Phase 4: Build & Playtest
1. **Deploy Script:**
   - Place `main.lua` into `CompleteStory/scripts/` (the active mod directory).
2. **User Verification:**
   - The user will launch the game, navigate to Episode Battle, and verify visually that the orange "Unlock" button has been replaced by the "New Game" badge.
3. **Execution Verification:**
   - The user will click "New Game" and confirm that the game correctly transitions to the Raditz opening mission instead of the DLC store modal.
