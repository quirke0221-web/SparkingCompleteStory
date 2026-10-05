# `retoc` Usage Guide for Dragon Ball: Sparking! ZERO

**Pinned:** retoc `0.1.5` · **Game engine:** UE `5.1.1` (per `docs/BUILD.md`, `docs/LOCAL_INPUTS.md`)

Epistemic labels (see `AGENTS.md` §1.3):
- `[FACT]` = backed by retoc source; see [`cli-reference.md`](cli-reference.md) for the line refs.
- `[LOCAL]` = backed by a script in this repo.
- `[POLICY]` = a project rule that is stricter than retoc requires.
- `[HYPOTHESIS]` = unproven.

---

## 1. Targeted Extraction (`to-legacy`)

Local implementation: `helpers/Build-CompleteStory.ps1` (Stage 1).

```powershell
& $retoc --aes-key $aes to-legacy --version UE5_1 `
    --filter 'SparkingZERO/Content/SS/Blueprints/DragonAdventureIFData' `
    $paksDir $outputDir
```

- `[FACT]` `--aes-key` is a global option and goes before `to-legacy`.
- `[LOCAL]` The key comes from `$env:SPARKING_ZERO_AES_KEY`. Errors redact arguments so
  the key is never echoed (`helpers/Common.ps1` L34-L45).
- `[OBSERVATION]` The local pipeline needs an AES key to read Sparking! ZERO's stock containers
  (`helpers/Build-CompleteStory.ps1` L38-L41). Upstream makes no game-specific statement.
- `[FACT]` `--filter` is a substring match. `DragonAdventureIFData` would also match a
  hypothetical `DragonAdventureIFDataFoo`. Use full package paths.
- `[LOCAL]` Filters use the on-disk path (`SparkingZERO/Content/...`), not `/Game/...`.
- `[FACT]` Input may be one `.utoc` or a directory. `[POLICY]` Pass the whole `Paks` directory.
  `[HYPOTHESIS]` A single `.utoc` would fail to resolve dependencies. Untested.
- `[FACT]` `--version` is optional for `to-legacy`. `[POLICY]` Always pass `UE5_1`.
- `[FACT]` `scriptobjects.bin` is written to the output root (UE 5.1 TOC > `PerfectHash`).
  `[POLICY]` Keep it for the packing step.
- `[FACT]` Per-package failures do not change the exit code. Always read the
  `Extracted N (M failed)` summary and require `M = 0`.

---

## 2. Container Compilation (`to-zen`)

Local implementation: `helpers/Build-CompleteStory.ps1` (Stage 5).

```powershell
& $retoc to-zen --version UE5_1 $stageDir (Join-Path $distDir 'CompleteStory_P.utoc')
```

Required staging layout:

```text
<stageDir>\
├── scriptobjects.bin                  # [POLICY] required by helpers/Build-CompleteStory.ps1 L92-L94
└── SparkingZERO\Content\...           # [FACT] mount point is ../../../ (game-root relative)
    ├── <Package>.uasset
    └── <Package>.uexp                 # [FACT] .uasset without .uexp is silently skipped
```

- `[FACT]` `--version` is required.
- `[FACT]` `scriptobjects.bin` is **optional upstream**. retoc only parses it, if present,
  "for VNI support and import checking".
- `[POLICY]` Still required here: `helpers/Build-CompleteStory.ps1` fails without it.
  `[HYPOTHESIS]` A container built without it fails in-game. Untested.
- `[FACT]` Outputs `<name>.utoc`, `<name>.ucas`, and a sibling `<name>.pak`.
- `[LOCAL]` `helpers/Build-CompleteStory.ps1` L103-L107 asserts all three exist.
- `[FACT]` Treat any `Skipping ... does not have a split exports file` log line as a failure.

Deployment location, `_P` naming policy, and which files a release must contain belong
to the `unverum-mod-packager` skill and its reference folder, not to retoc.

---

## 3. Verification (`verify`)

Local implementation: `helpers/Build-CompleteStory.ps1` (Stage 6).

```powershell
& $retoc verify (Join-Path $distDir 'CompleteStory_P.utoc')
```

- `[FACT]` Passing means each chunk in `.ucas` matches its TOC hash. Output: `verified`.
- `[FACT]` It does **not** prove the game will mount or load the container.
  Only an in-game test proves that.
- `[OBSERVATION]` `docs/VERSION_HISTORY.md` (v0.1) records a retoc-built container passing
  `verify` and loading in-game, where it caused a soft-lock.

---

## 4. Traps

| Trap | Receipt | Correct procedure |
|---|---|---|
| Using `UnrealPak` or another packer | Not in the approved matrix | `retoc to-zen` only |
| `to-zen` without `--version` | `[FACT]` required arg | `--version UE5_1` |
| `--aes-key` after the subcommand | `[FACT]` global arg | `retoc --aes-key ... to-legacy ...` |
| Trusting `to-legacy` exit code 0 | `[FACT]` failures are counted, not fatal | Parse `(M failed)` |
| Loose `--filter` strings | `[FACT]` substring match | Full package paths |
| `.uasset` staged without `.uexp` | `[FACT]` silently skipped | Pre-pack pairing check |
| Reading `verify` as "game will load it" | `[FACT]` hash check only | In-game test |
| Writing to the game folder or `~mods/` while extracting | `[POLICY]` | Scratch/staging dirs only |
| Hardcoding or echoing the AES key | `[LOCAL]` redaction in `Common.ps1` | Env var only |
