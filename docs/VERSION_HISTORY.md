# Version and Failure History

| Version | Change and expectation | Actual runtime result | Offline result / source | Do not repeat |
|---|---|---|---|---|
| v0.1 | Added thirteenth character-registry entry and cloned Goku data; expected Raditz. | Tile appeared, said `Unlock`, Confirm soft-locked navigation and Back. | Two-package IoStore passed parsing and `retoc verify`. Transform source remains in `scripts/build_complete_story_assets.ps1`; release hashes archived. | A character registry record alone is insufficient. |
| v0.2 | Added `IgnoreOpen=true` to bypass locking. | Complete Story disappeared. | Serialization/package validation passed; local JSON/binaries are excluded. | Do not set `IgnoreOpen=true`. |
| v0.3 | Added matching chart-registry record reusing Goku's `ChartData0000_00`. | All twelve stock entries plus Complete Story appeared. Confirm opened the NEO storefront; still `Unlock`. | Clean recommended baseline; three packages, imports, and IoStore verified. | Do not call this playable. |
| v0.4 | Changed `DefaultOpenCharacter` from `0000_40` to `0000_00`. | Selector initialization regressed and other characters disappeared. | Byte/property comparison isolated the default-key change. | Keep `0000_40`. |
| v0.5 | Added `0000_00` to DLC 013 `AdventureIFCharacterIds`. | Same `Unlock` and same NEO storefront. | Edit serialized and packaged. Evidence summary retained. | Do not modify DLC records or infer ownership is the cause. |
| v0.6 | Added UE4SS hooks for `IsPlayable`, `DecideButton`, and `NewDecideButton`; attempted to identify selection through widget text/global searches. | Initial component activation was wrong; after correction hooks registered, then Episode Battle crashed. Exact native crash site was not proven. | Source preserved under `archive/legacy/v0.6-runtime`. Log proved registration, not a completed override. | No global `FindAllOf`, `StaticFindObject`, UObject/FText scanning in hot callbacks. |
| v0.7 | Delayed one `IsPlayable` hook by 60 seconds and inspected six panels, expecting an index. | Startup remained stable; on Episode Battle callback entry it logged `-3/6,-2/4,-1/2,1/2,2/4,3/6`, then crashed. | Source preserved under `runtime/v0.7`; sanitized log retained. | Values are relative carousel positions, not registry indexes. Do not reflect panels in the callback. |

No version has launched Raditz through Complete Story. Cross-character chronology is not implemented.
