# Local Inputs and Exclusions

This public repository deliberately excludes content that cannot safely or legally be published.

Owner-supplied inputs:

- Legitimate Steam install, build `24953175`, Unreal Engine `5.1.1`.
- Stock main IoStore container from `SparkingZERO/Content/Paks`.
- AES key supplied only through environment variable `SPARKING_ZERO_AES_KEY`.
- UAssetGUI mapping named `SparkingZERO`; the used mapping SHA-256 is `B7AE00F54BA558EF7793CABA3437B3E82D64D29A6822C34CE73B4A1F91B2D1C5`.
- Locally generated `scriptobjects.bin`.

Known tool versions: FModel `4.4.4.0`, retoc `0.1.5`, UAssetGUI/UAssetAPI `1.1.0`, UE4SS `3.0.1 Beta`.

Excluded paths include `build/` and `work/` game-derived assets, `dist/` cooked packages, `tools/` third-party binaries, all save backups, AES/config secrets, full object dumps, minidumps, and stock `global.utoc/.ucas`. Exact local classifications are in `docs/FILE_AUDIT.md`.
