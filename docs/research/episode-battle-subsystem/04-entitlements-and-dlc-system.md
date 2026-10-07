# Episode Battle Subsystem: Entitlements & DLC Licensing System

> **Status:** AUDITED RESEARCH RECORD  
> **Target Subsystem:** DLC Licensing, Store Triggers & Entitlement Checks  
> **Source Receipts:**  
> - `evidence/assets/structural-summary.md` (commit `e719f6b0`, v0.5 DLC experiment)  
> - `evidence/reproduction/required-assets.txt` (commit `e719f6b0`, lines 12–16)  
> - In-game observed storefront modal (*"Super Limit-Breaking NEO"*)  

---

## 1. Why the Steam DLC Store Modal Triggers

One of the most persistent symptoms encountered during our testing was:
> When hovering over or confirming the locked Complete Story tile, the game pops up the Steam DLC overlay prompt (*"Super Limit-Breaking NEO"* / Season Pass modal).

### The Root Cause: Fallback Store Dispatch
* `[FACT]`: In *Dragon Ball: Sparking! ZERO*, the Episode Battle carousel is designed to support post-launch DLC characters (e.g. DAIMA, Super Hero characters).
* `[FACT]`: In `SSBuiltInMenu`, when an entry is in `EKoratUnLockMode::Lock` state, the button prompt changes from "Start" (`NewDecideButton`) to "Unlock" (`DecideButton`).
* `[FACT]`: Pressing `DecideButton` on a locked character invokes the game's entitlement check:
  - If the character belongs to a DLC pack (or if the engine detects a locked extra saga), it dispatches the platform storefront URI (Steam overlay) to purchase the corresponding entitlement.
  - Therefore, the DLC modal is **not a bug**—it is the engine's intended fallback behavior for any character that fails the `IsPlayable` check!

---

## 2. DLC Asset Structure (`DownLoadContents`)

The DLC registry is defined by a master Blueprint and child DataAssets:

```text
SparkingZERO/Content/SS/
├── Blueprints/
│   └── DownLoadContentsData.uasset           <-- Master DLC Registry
└── MasterDataAsset/DownLoadContents/
    ├── DLC_010.uasset                        <-- Pre-Order Bonus / Goku (Mini)
    ├── DLC_011.uasset                        <-- Season Pass Pack 1
    ├── DLC_012.uasset                        <-- Season Pass Pack 2
    └── DLC_013.uasset                        <-- Season Pass Pack 3
```

### 2.1 Why Custom Sagas Cannot Be Registered as DLC
* DLC packages in *Dragon Ball: Sparking! ZERO* declare character IDs under `AdventureIFCharacterIds`.
* However, all entries declared in `DownLoadContentsData` require validation against Steam's remote license server.
* Because Steam only validates official product licenses, any custom character key registered under a DLC container will fail Steam entitlement validation and trigger the store popup.
* `[CONCLUSION]`: Custom campaigns **must NOT be registered as DLC**. They must be registered purely as native base-game content.

---

## 3. The Pure Base-Game Campaign Pattern

To bypass the platform storefront trigger completely, a custom campaign must be registered as a **native base-game saga**, identical to Vanilla Goku (`0000_40`):

1. **Keep out of `DownLoadContentsData`:** Do not add `0000_00` to any DLC DataAsset.
2. **Declare in `DragonAdventureIFData.PtrRecords`:** Register purely as a native carousel entry.
3. **Satisfy `IsPlayable` and `EKoratUnLockMode::New`:**
   - Either via `DefaultOpenCharacter = "0000_00"` in `DragonAdventureIFData`.
   - Or via native ABI execution detour on `execIsPlayable` returning `true`.
4. **Result:** When `IsPlayable` returns `true`, the UI sets the state to `EKoratUnLockMode::New`, replaces `DecideButton` with `NewDecideButton`, and routes input to `IsModeStart()`, launching Chapter 1 without ever querying the Steam DLC system.

---

## 4. Epistemic Assessment

* `[FACT]`: The Steam DLC popup is the native fallback behavior when confirming a tile in `EKoratUnLockMode::Lock`.
* `[FACT]`: Adding custom keys to DLC packages fails because Steam owns the DLC license database.
* `[FACT]`: The only way to stop the Steam store popup is to transition the tile state to **Unlocked (`EKoratUnLockMode::New`)**, which swaps the confirmation handler to `IsModeStart()`.
