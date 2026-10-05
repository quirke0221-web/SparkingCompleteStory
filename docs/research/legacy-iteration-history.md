# Complete Story: Legacy Prototype Iteration & Failure History (v0.1 – v0.7)

> **Status:** AUTHORITATIVE POST-MORTEM  
> **Source Evidence:** Empirical game logs, sanitized runtime traces, asset deltas in `history/`, and RE-UE4SS crash diagnostics.

---

## 1. Executive Summary

Between 2026-10-02 and 2026-10-04, seven iterative prototypes (v0.1 through v0.7) were tested to establish a playable thirteenth Episode Battle campaign for Goku (Mini). While earlier iterations successfully solved additive IoStore serialization and UI tile rendering (v0.3 baseline), attempts to force unlock states via quick hacks resulted in menu regressions and fatal engine crashes.

This document serves as the permanent post-mortem record to prevent future contributors or AI agents from repeating disproven hypotheses.

---

## 2. Iteration Chronology & Failure Post-Mortem

### v0.1: Character Registry Extension (Offline Success, Runtime Soft-Lock)
* **Hypothesis:** Appending a thirteenth entry to `DragonAdventureIFData.PtrRecords` and cloning Goku's character data will render and launch the campaign.
* **Implementation:** Appended `0000_00 -> DAIF_CharaData_CompleteStory` in `PtrRecords`. Cloned Goku's asset.
* **Observed Result:** Thirteenth tile rendered in Episode Battle with label `Unlock`. Pressing Confirm soft-locked the UI (navigation and Back button ceased functioning).
* **Root Cause:** Episode Battle requires dual registration: character data *and* chart data. Without a chart mapping, the engine cannot resolve travel destinations.

### v0.2: Property Unlock Bypass Attempt (Complete Disappearance)
* **Hypothesis:** Setting `IgnoreOpen = true` on `DAIF_CharaData_CompleteStory` will bypass lock checks.
* **Implementation:** Toggled property `IgnoreOpen` to `true` in serialized JSON.
* **Observed Result:** Complete Story tile completely vanished from the character selection carousel.
* **Root Cause:** `IgnoreOpen` is an internal flag used to filter unreleased or debug entries from the presentation layer.

### v0.3: Dual Chart Registration (The Clean Proven Asset Baseline)
* **Hypothesis:** Adding a matching entry in `DragonAdventureIFChartData` pointing to Goku's `ChartData0000_00` will satisfy travel resolution.
* **Implementation:** Appended `0000_00 -> ChartData0000_00` in chart registry. Retained `DefaultOpenCharacter = 0000_40`.
* **Observed Result:** All 12 stock campaigns intact; Complete Story visible as 13th tile. Tile displayed `Unlock`. Pressing Confirm opened the unrelated NEO storefront popup.
* **Significance:** **Cleanest asset baseline.** Proved dual-registry asset stability. Established that missing native playability state routes to a generic store fallback.

### v0.4: Startup Identity Mutation (Menu Selector Regression)
* **Hypothesis:** Changing `DefaultOpenCharacter` from `0000_40` to `0000_00` will force the engine to initialize the new key as unlocked.
* **Implementation:** Mutated `DefaultOpenCharacter` in `DragonAdventureIFData`.
* **Observed Result:** Broke selector initialization; stock character tiles disappeared or failed to focus.
* **Root Cause:** `DefaultOpenCharacter` is menu startup state, not an unlock initializer. Stock value `0000_40` must never be altered.

### v0.5: DLC Entitlement Hypothesis (Disproven)
* **Hypothesis:** The NEO storefront popup indicates a missing DLC license; adding `0000_00` to DLC tables will unlock it.
* **Implementation:** Appended `0000_00` to DLC 013 `AdventureIFCharacterIds`.
* **Observed Result:** Zero behavioral change. The `Unlock` label and NEO storefront popup persisted identically.
* **Root Cause:** The storefront popup is an unhandled exception fallback, not a true DLC entitlement check.

### v0.6: Unsafe Runtime Hooking & Global Slate Scraping (Native Engine Crash)
* **Hypothesis:** RE-UE4SS can intercept `IsPlayable`, `DecideButton`, and `NewDecideButton` and identify selection by searching for widget text.
* **Implementation:** Script called `FindAllOf("TextBlock")` and reflected UI widgets inside execution callbacks.
* **Observed Result:** Fatal game crash upon opening Episode Battle.
* **Root Cause:** Slate widgets are transient and pooled on the rendering thread. Scraping widgets in hot callbacks causes dangling pointer dereferences and `0xC0000005` access violations.

### v0.7: Delayed Hook Installation & Carousel Reading (Access Violation Post-Mortem)
* **Hypothesis:** Delaying hook installation by 60 seconds and reading 6 carousel panel indices will identify the selected campaign safely.
* **Implementation:** Delayed `IsPlayable` hook until GameThread stabilized; read panel `Index/ShowNum` properties.
* **Observed Result:** At Episode Battle entry, logged panel values: `-3/6, -2/4, -1/2, 1/2, 2/4, 3/6`, then immediately crashed (`0xC0000005`).
* **Root Cause:** The numbers logged were **relative circular carousel offsets** around the selected center, not absolute registry indices. Attempting to match index `12` failed, and reflecting panel widgets inside native callbacks dereferenced pooled memory.

---

## 3. "Do Not Repeat" Invariant Matrix

| Anti-Pattern Attempted | Version | Failure Mechanism | Mandated Architecture |
| :--- | :--- | :--- | :--- |
| Single-registry registration | v0.1 | Soft-locks UI navigation | Must register in both `DragonAdventureIFData` and `ChartData`. |
| Setting `IgnoreOpen = true` | v0.2 | Hides tile from carousel | Leave `IgnoreOpen = false`. |
| Changing `DefaultOpenCharacter` | v0.4 | Breaks menu carousel initialization | Strictly preserve `DefaultOpenCharacter = 0000_40`. |
| Editing DLC 013 records | v0.5 | Disproven; does not affect unlock | Leave stock DLC definitions untouched. |
| Slate widget reflection (`FindAllOf`) | v0.6 | Dereferences pooled Slate widgets | Strictly banned (ADR 0004). Never scrape UI widgets. |
| Carousel panel index scanning | v0.7 | Values are relative circular offsets | Hook stable UFunction `IsPlayable()` on GameThread. |
