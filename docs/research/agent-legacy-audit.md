# Research: Legacy Agent Audit & Post-Mortem

**Date:** 2026-10-04  
**Subject:** Codebase and Documentation Audit of Prior AI Agent Implementations  
**Status:** Complete  

---

## 1. Executive Summary

This repository was previously generated through iterative prompting of an AI coding agent. While the agent made legitimate progress in reverse-engineering Unreal Engine 5's IoStore asset format and adding a visual tile to the Episode Battle carousel, it suffered from **architectural sprawl**, **code duplication**, and **fundamentally flawed runtime assumptions**.

This audit separates the verified, working technical assets from the dead ends and hallucinations.

---

## 2. Organizational Deficiencies (What the Agent Did Wrong)

### A. Root Directory Saturation (Document Sprawl)
The agent created 10 separate markdown and CSV documentation files at the root level:
* `PROJECT_STATE.md`, `DEVELOPMENT_HISTORY.md`, `BUILD_REPORT.md`, `PACKAGING_BLOCKER.md`, `FINDINGS.md`, `ASSET_MAP.md`, `ASSET_INVENTORY.csv`, etc.
* **The Issue:** Many of these were transient "snapshot" notes written during earlier attempts (e.g., v0.1), which became obsolete once subsequent versions were tested. Instead of updating a centralized document, the agent kept creating new files, resulting in conflicting statuses across the repository.

### B. Byte-for-Byte Tree Duplication (~730 Duplicate Lines)
The agent left exact duplicate working trees in multiple places:
1. `mods/CompleteStory13thTest/` is 100% byte-for-byte identical to `archive/legacy/CompleteStory13thTest/` (227 lines of Lua).
2. `runtime/CompleteStory/` is 100% byte-for-byte identical to `archive/legacy/v0.6-runtime/` (225 lines of Lua).
3. `INSTALL.txt` was duplicated across three separate folders.

### C. Monolithic Scripting ("God Scripts")
* `scripts/build_complete_story_assets.ps1` (172 lines) was written as a single monolithic script that handles JSON file I/O, domain assertion checks, deep cloning of game structures, import-table recalculations, and output writing without modular separation.

---

## 3. What the Agent Got Right (The Proven Baseline: v0.3)

The agent succeeded in reverse-engineering the game's asset serialization layer and creating a working IoStore package:

1. **Extraction & Packaging Pipeline:**
   * Leveraged `retoc` and `UAssetAPI` to cleanly deconstruct UE 5.1 Zen IoStore containers into JSON and rebuild them back into valid `.pak/.utoc/.ucas` containers.
2. **Registry Injection (`DragonAdventureIFData`):**
   * Appended key `0000_00` to `PtrRecords` while preserving all 12 original character keys in their exact stock order and maintaining `DefaultOpenCharacter = 0000_40`.
3. **Chart Mapping (`DragonAdventureIFChartData`):**
   * Cloned Goku's working chart record (`ChartData0000_00`) under key `0000_00`, correctly referencing Goku's canonical Raditz start events (`Event_00_0_00_00`, `EventBlock_0000_00`, `DIF_Event_0000_00`).
4. **In-Game Verification:**
   * In-game testing confirmed that the 13th tile titled **"Complete Story"** renders cleanly in the Episode Battle carousel alongside all 12 stock characters without breaking menu navigation.

---

## 4. What the Agent Got Wrong (Runtime Fallacies & Crashes)

### A. Misunderstanding the "NEO Storefront" (v0.5 Blunder)
* **What Happened:** When selecting "Complete Story" in-game, the tile showed an `Unlock` prompt. Pressing confirm opened a store page for the "Super Limit-Breaking NEO" DLC.
* **The Agent's Mistake:** The agent hypothesized that Complete Story was lacking DLC entitlements, so in v0.5 it attempted to append `0000_00` to DLC 013 (`DownLoadContentsData`).
* **The Reality:** In *Sparking! ZERO*, the NEO storefront popup is simply the game's generic fallback screen when any campaign or character lacks an unlocked state in the save system. It had nothing to do with DLC entitlements.

### B. Unsafe Slate/UMG UI Screen-Scraping (v0.6 Blunder)
* **What Happened:** In v0.6 (`archive/legacy/v0.6-runtime/Scripts/main.lua`), the agent tried to hook `IsPlayable`, `DecideButton`, and `NewDecideButton`, and search the entire widget tree using `FindAllOf("TextBlock")` to see if the active text on screen matched `"Complete Story"`.
* **The Reality:** In Unreal Engine, Slate UI widgets are dynamically recycled and do not guarantee safe memory lifespans across frames. Calling broad UObject search functions inside UI callbacks caused immediate native memory access violations (fatal crashes).

### C. Carousel Position Hallucination (v0.7 Blunder)
* **What Happened:** In v0.7 (`runtime/v0.7/CompleteStory/Scripts/main.lua`), the agent attempted to inspect panel widgets and logged the following values:
  ```text
  -3/6, -2/4, -1/2, 1/2, 2/4, 3/6
  ```
* **The Reality:** The agent incorrectly assumed these numbers were character registry indexes and tried to match them against index `12`. In truth, these values are relative carousel layout coordinates (representing visible carousel slots). Reading them triggered another crash on entering Episode Battle.

---

## 5. Architectural Takeaways for Moving Forward

1. **Retain the v0.3 Asset Transformer:** The asset editing approach is sound, verified, and correctly mounts into UE 5.1.
2. **Discard UI Widget Scraping:** Completely abandon Lua scripts that inspect or scrape Slate/UMG widgets.
3. **Focus on Native Save Data Lifecycle:** The reason Complete Story shows `Unlock` is that the game's save system (`FSSDragonAdventureIFSaveData::CharacterPlayableData`) does not have a record for key `0000_00`. Research must focus on how the game loads this save data into memory.
