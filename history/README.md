# Source Revision Index

This index ties each tested build to its actual preserved source or exact authored asset delta. Full UAssetAPI JSON snapshots are local-only because they reproduce game-owned serialized assets.

| Version | Public source/delta | Local full development files |
|---|---|---|
| v0.1 | `v0.1/asset-delta.json`; core transform in `scripts/build_complete_story_assets.ps1` before chart section | `build/json`, `build/legacy`, `build/validation`, `dist/CompleteStory-v0.1*` |
| v0.2 | `v0.2/asset-delta.json` | `build/v0.2`, `dist/CompleteStory-v0.2*` |
| v0.3 | `v0.3/asset-delta.json`; current complete transform script | `build/v0.3`, `dist/CompleteStory-v0.3*` |
| v0.4 | `v0.4/asset-delta.json` | `build/v0.4`, `dist/CompleteStory-v0.4*` |
| v0.5 | `v0.5/asset-delta.json` | `build/v0.5`, `dist/CompleteStory-v0.5*` |
| v0.6 | Actual Lua at `archive/legacy/v0.6-runtime/Scripts/main.lua` | `dist/CompleteStory-v0.6*`, test backups |
| v0.7 | Actual Lua at `runtime/v0.7/CompleteStory/Scripts/main.lua` | `dist/CompleteStory-v0.7*`, `work/runtime_tests` |

The JSON delta files describe only project-authored changes and hashes observed from the retained outputs. They are not invented substitutes for missing scripts: v0.1–v0.5 were produced by evolving the asset JSON/pipeline directly, and no separate historical PowerShell scripts were recovered.
