# Packaging Blocker — Resolved

Date resolved: 2026-10-02

The original blocker was authorized read-only access to the encrypted stock
IoStore. The user supplied the working key for Steam build `24953175`. The key
is intentionally not repeated or stored in this workspace documentation.

## Resolution

- `retoc info` successfully read the encrypted base container.
- Targeted `to-legacy` conversion extracted only
  `DragonAdventureIFData` and `DAIF_CharaData_0000_00`.
- UAssetAPI parsed both assets with the exact build mapping.
- The new character package and additive registry override were serialized as
  real cooked assets, not edited FModel JSON/blob output.
- `retoc to-zen --version UE5_1` created the final `.pak/.utoc/.ucas` set.
- `retoc verify` passed.
- A final decode with the base container available resolved all package and
  object imports without `UnknownPackage` or `UnknownExport` records.

## UE5.1 limitation handled

When a repacked registry was decoded in isolation, retoc could preserve package
IDs but could not name base packages that were absent from the validation
directory. This matches retoc's pre-UE5.3 dependency limitation. A full-context
decode using read-only workspace hard links to the base container resolved all
12 original route imports plus the new route import. The temporary links were
used only for validation and are not part of the distribution.

## Current package

`dist/CompleteStory-v0.1/CompleteStory_P.{pak,utoc,ucas}`

Runtime behavior remains untested because the game was not launched. See
`PROJECT_STATE.md` and `dist/CompleteStory-v0.1/README.md`.
