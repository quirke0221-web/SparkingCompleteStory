# Complete Story Runtime Module (RE-UE4SS)

Authoritative runtime scripting component for the **Sparking ZERO: Complete Story** mod.

## Overview
- **Framework:** [RE-UE4SS](https://github.com/UE4SS-RE/RE-UE4SS) `v3.0.1 Beta` (commit `4e5461c`)
- **Target Game:** *Dragon Ball: Sparking! ZERO* (PC / Steam build `24953175`, Unreal Engine `5.1.1`)
- **Entrypoint:** `scripts/main.lua`
- **Activation Flag:** `enabled.txt`

## Architecture & Guarantees
1. **GameThread Safe:** Hooks `/Script/SS.SSDragonAdventureIFCSManager:IsPlayable` safely via `RegisterHook`.
2. **Zero Slate/UMG Scraping (ADR 0004):** Strictly bans transient widget reflection (`FindAllOf`), UI panel walking, and text scraping.
3. **Non-Destructive Coexistence:** Overrides `ReturnValue` to `true` strictly for route key `0000_00` (Complete Story). Returns `nil` for all 12 stock campaigns to preserve vanilla save validation.

## Deployment
- Staged automatically to `<GameRoot>\SparkingZERO\Binaries\Win64\Mods\CompleteStory` via:
  ```powershell
  .\scripts\Deploy-DevelopmentBuild.ps1 -Install
  ```
