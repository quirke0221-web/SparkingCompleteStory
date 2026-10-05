# Reproducible Build Workflow

## Compatibility

The established pipeline targets Steam build `24953175`, UE `5.1.1`, FModel `4.4.4.0`, retoc `0.1.5`, UAssetGUI/UAssetAPI `1.1.0`, and mapping SHA-256 `B7AE00F54BA558EF7793CABA3437B3E82D64D29A6822C34CE73B4A1F91B2D1C5`.

## Setup

Copy `config/project.local.example.psd1` to ignored `config/project.local.psd1`, update local paths, and set `SPARKING_ZERO_AES_KEY` in the current shell. Never write the key into a script or commit it.

## End-to-end stages

1. Use retoc `to-legacy --version UE5_1` against the legitimate stock IoStore, filtering only the package paths in `evidence/reproduction/required-assets.txt`. Retain the generated `scriptobjects.bin` in local staging. Do not extract the whole game.
2. Run `scripts/Export-AssetJson.ps1` to export the two registries and Goku character data with UAssetGUI.
3. Run `scripts/build_complete_story_assets.ps1` with the three source JSON paths. It asserts the twelve-entry baseline, `DefaultOpenCharacter=0000_40`, Goku's Raditz references, then emits the exact v0.3 transform: a cloned character asset plus thirteenth character/chart records.
4. Run `scripts/Test-CompleteStoryJson.ps1` for structural assertions.
5. Run `scripts/Import-ModifiedAssets.ps1` to serialize mapped UE5.1 legacy assets into staging. Copy the locally generated `scriptobjects.bin` to the staging root.
6. Run `scripts/Build-IoStore.ps1` to convert staging back to a UE5.1 IoStore overlay.
7. Run `scripts/Verify-IoStore.ps1`; also decode the result in full stock-container context and re-export JSON to compare records, imports, and start pointers.
8. Package only `CompleteStory_P.pak/.utoc/.ucas`. Generated ZIPs are intentionally excluded from Git.

## Important limitation

These scripts reproduce the **v0.3 asset baseline**, which is known to remain locked in game. They do not solve native playability. The targeted extraction command still requires the owner-supplied AES key and stock packages, so CI cannot perform a full clean build. UAssetGUI 1.1.0 and retoc's UE5.1 conversion were validated locally, but a fresh clone cannot build without those excluded inputs.

Install and uninstall helpers support `-WhatIf`; read `docs/TESTING.md` before touching a live installation.
