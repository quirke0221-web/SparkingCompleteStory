# Derived Asset Evidence

## Registry revisions

All versions preserved the twelve stock keys:

`0000_40,0020_60,0032_00,0050_00,0040_00,0060_00,0070_00,0080_30,0153_00,0162_00,0800_00,0930_00`

v0.1/v0.3/v0.5 append `0000_00` and retain `DefaultOpenCharacter=0000_40`. v0.2 additionally sets the cloned character's `IgnoreOpen=true`. v0.4 alone changes `DefaultOpenCharacter` to `0000_00`; its registry JSON SHA differs while v0.1/v0.3/v0.5 share the same registry hash.

The clean registry JSON SHA-256 is `CEDFE3C54C721840785113522D7EB6CD41889DC173C6769EA319CEE26BA0E3AF`. v0.4 is `27F861302B47CE0C646DB8AC82F1185256CF149324BC6EB8073DECE765DABA88`.

## v0.5 DLC experiment

The sole semantic v0.5 DLC edit was appending `0000_00` to DLC 013 `AdventureIFCharacterIds`. Stock values there were `0050_00`, `0060_00`, `0070_00`, and `0162_00`; `DlcUnlockIgnoreCharacterIds` contained `0310_00` and `0020_50`. Runtime behavior remained `Unlock` plus the same storefront popup.

## Start references

`DAIF_CharaData_CompleteStory` was cloned from Goku and retains:

- `Event_00_0_00_00`
- `EventBlock_0000_00`
- `DIF_Event_0000_00`
- chart object `ChartData0000_00`

Full JSON and binaries are excluded as game-derived content. Recreate them with `docs/BUILD.md`.
