---
name: retoc-iostore-packer
description: Mandatory before ANY retoc invocation. Use when extracting Dragon Ball Sparking! ZERO packages from IoStore (.utoc/.ucas) to legacy .uasset/.uexp, packing staged legacy assets into a UE 5.1 IoStore container (.utoc/.ucas/.pak), verifying a container, or inspecting container contents.
---

# Skill: retoc IoStore Packer

> **Status:** AUDITED & ACTIVE (Phase 1 Passed)  
> **Pinned version:** retoc `0.1.5` (workspace `version` in upstream `Cargo.toml`).
> **Audited source commit:** [`885a8da`](https://github.com/trumank/retoc/tree/885a8dae740cb1ce1e41ff2e74f67f9f0c118237)
> **Reference folder:** [`docs/dependencies/retoc/`](../../../docs/dependencies/retoc/README.md)
> (verbatim source excerpts: `source-receipts-arguments.md`, `source-receipts-commands.md`, `source-receipts-behavior.md`)

Every rule below cites its receipt. `[S:Lx]` = line `x` of
[`retoc_cli/src/main.rs` @ 885a8da](https://github.com/trumank/retoc/blob/885a8dae740cb1ce1e41ff2e74f67f9f0c118237/retoc_cli/src/main.rs).
`[LOCAL]` = this repo's existing scripts. If a behavior is not cited here, it is
not verified. Do not invent it. Research it first (see `AGENTS.md` §1).

---

## 1. Scope Boundaries

**Use this skill for:**
- `to-legacy`: targeted extraction of stock packages from the game's IoStore.
- `to-zen`: compiling a staged legacy asset tree into a mod container.
- `verify`, `info`, `list`, `manifest`: read-only container checks.

**Do NOT use this skill for (hand off instead):**
| Task | Owner skill |
|---|---|
| Editing `.uasset` content / JSON conversion | `uassetgui-asset-serialization` |
| Deploying containers into `~mods/`, release ZIP layout, `_P` naming policy | `unverum-mod-packager` |
| Signature-check bypass runtime prerequisite | `utoc-signature-bypass` |
| End-to-end build ordering | `sparking-zero-mod-pipeline` |

**Never use** `UnrealPak`, `repak`, or any other packer for Sparking! ZERO mod
containers. Only retoc is approved in `docs/dependencies/dependencies.md`.

---

## 2. Verified Commands (Golden Paths)

### 2.1 Targeted extraction: `to-legacy`

```powershell
# $retoc   = configured RetocPath (config/project.local.psd1)
# $paks    = "<GameRoot>\SparkingZERO\Content\Paks"
# $aes     = value of $env:SPARKING_ZERO_AES_KEY (never hardcode, never echo)
& $retoc --aes-key $aes to-legacy --version UE5_1 `
    --filter 'SparkingZERO/Content/SS/Blueprints/DragonAdventureIFData' `
    $paks $outputDir
if ($LASTEXITCODE -ne 0) { throw 'retoc to-legacy failed (arguments redacted)' }
```

Facts behind each part:
- `--aes-key` belongs to the top-level `Args` struct, not the subcommand
  `[S:L279-L281]`. Place it **before** `to-legacy`, as the validated local script
  does `[LOCAL: scripts/Extract-RequiredAssets.ps1:22]`.
- Input may be a single `.utoc` **or a directory of containers** (e.g. `Content/Paks`) `[S:L107-L110]`.
  Project policy: always pass the `Paks` directory `[LOCAL]`.
- `--version` is **optional** for `to-legacy` (`Option<EngineVersion>`) `[S:L137]`.
  Project policy: always pass `UE5_1` `[LOCAL]`.
- `--filter` is repeatable (`Vec<String>`) `[S:L117]` and uses **substring**
  matching against the container path: `package_path.contains(f)` `[S:L692]`.
  It is NOT a prefix, glob, or regex. Several `--filter` flags may go in one call.
- Filter against the **on-disk package path** (`SparkingZERO/Content/...`), not the
  `/Game/...` object path `[LOCAL: Extract-RequiredAssets.ps1:5-7]`.
- Output selection: if `<OUTPUT>` ends in `.pak` a pak is written, otherwise a
  directory tree `[S:L628]`. Project policy: extract to a directory.
- Unless `--no-script-objects` is set, a `scriptobjects.bin` is written to the
  output root when the TOC version is above `PerfectHash` `[S:L673-L677]`.
  `UE5_1` maps to `PerfectHashWithOverflow` (`retoc/src/version.rs:32`), which is above
  `PerfectHash` in `EIoStoreTocVersion` (`retoc/src/lib.rs:1164`). So this file **will** be produced.

### 2.2 Container compilation: `to-zen`

```powershell
& $retoc to-zen --version UE5_1 $stageDir (Join-Path $distDir 'CompleteStory_P.utoc')
if ($LASTEXITCODE -ne 0) { throw 'retoc to-zen failed' }
```

Facts:
- `--version` is **required** for `to-zen` (non-optional `EngineVersion`) `[S:L169]`.
- Input is a directory or a `.pak` `[S:L155-L160]`.
- The mount point is hard-coded to `../../../` `[S:L769]`. The staging tree must
  therefore mirror the game layout from its root: `<stageDir>\SparkingZERO\Content\...`.
  This matches what `to-legacy` writes (it strips `../../../`) `[S:L715]`.
- Only `.uasset`/`.umap` files **with a sibling `.uexp`** are packed. Otherwise the file is
  **skipped with an info message, not an error** `[S:L794-L801]`.
- Outputs: `<name>.utoc` + `<name>.ucas`, plus a `<name>.pak` written next to it
  `[S:L938-L940]`.
- `scriptobjects.bin`: **optional** upstream. If any file named `scriptobjects.bin`
  is in the input it is parsed "for VNI support and import checking" `[S:L807-L811]`.
  **Project policy: required.** Stage it at `<stageDir>\scriptobjects.bin` before
  packing `[LOCAL: scripts/Build-IoStore.ps1:11-12]`.

### 2.3 Verification: `verify`

```powershell
& $retoc verify (Join-Path $distDir 'CompleteStory_P.utoc')
if ($LASTEXITCODE -ne 0) { throw 'retoc verify failed' }
```

What `verify` actually checks: it reads every chunk from the sibling `.ucas` and compares
the first 20 bytes of a BLAKE3 hash with the TOC chunk hash. It prints `verified` on success
and fails with `hash mismatch for chunk #N` otherwise `[S:L438-L503]`.
It does **not** validate the directory index, package dependencies, signatures,
or whether the game will load the container.

### 2.4 Read-only inspection

- `retoc info <path>`: container info.
- `retoc manifest <utoc>`: extract manifest.
- `retoc list <utoc> [--all] [--hash] [--package] [--size] [--path] [--store]` `[S:L50-L72]`.

---

## 3. Pre-Flight Checklist (run BEFORE invoking)

- [ ] Version gate: `& $retoc --version` reports `0.1.5`. If not, stop. The version is pinned.
- [ ] `--version UE5_1` is present (required for `to-zen`, policy for `to-legacy`).
- [ ] **to-legacy:** `$env:SPARKING_ZERO_AES_KEY` is non-empty. The key is passed only by
      variable, and no log, error, or transcript echoes the argument list `[LOCAL: scripts/Common.ps1:26-33]`.
- [ ] **to-legacy:** each filter is as specific as possible (substring matching over-matches).
- [ ] **to-legacy:** output directory is a scratch/staging path and never `~mods/` or the game install.
- [ ] **to-zen:** staging root contains `scriptobjects.bin` and a `SparkingZERO\Content\...` tree.
- [ ] **to-zen:** every staged `.uasset` has a matching `.uexp`. Enumerate and compare before packing.
- [ ] **to-zen:** staging contains **only** packages that the mod intentionally overrides.

## 4. Post-Flight Checklist (run AFTER invoking)

- [ ] Exit code is `0`. A Rust `main` returning `Err` exits with `EXIT_FAILURE`
      ([Rust `Termination` docs](https://doc.rust-lang.org/std/process/trait.Termination.html)).
- [ ] **to-legacy:** exit code alone is NOT enough. Per-package conversion failures are
      logged and counted but **do not fail the process** `[S:L719-L723]`. Parse
      `Extracted N (M failed)` `[S:L738]` and require `M = 0` and `N` equal to the expected count.
- [ ] **to-legacy:** every expected `.uasset` + `.uexp` exists, and `scriptobjects.bin` exists at output root.
- [ ] **to-zen:** output contains **no** `Skipping ... does not have a split exports file` lines. Treat any as failure.
- [ ] **to-zen:** `.utoc`, `.ucas`, `.pak` all exist with non-zero size `[LOCAL: Build-IoStore.ps1:18-20]`.
- [ ] **to-zen:** `retoc verify` prints `verified`.
- [ ] **to-zen:** `retoc list <utoc> --path` shows exactly the intended package paths and nothing else.

---

## 5. Negative Constraints (Hallucination Defense)

| Forbidden | Why (receipt) | Do instead |
|---|---|---|
| Using `UnrealPak`/other packers | Not approved; retoc is the pinned toolchain | `retoc to-zen` |
| Omitting `--version` on `to-zen` | Argument is required `[S:L169]` | `--version UE5_1` |
| Putting `--aes-key` after the subcommand | It is a top-level arg `[S:L279-L281]` | `retoc --aes-key $aes to-legacy ...` |
| Treating `--filter` as prefix/glob/regex | Substring `contains()` `[S:L692]` | Use the most specific path |
| Trusting exit code 0 from `to-legacy` | Per-package failures don't fail the process `[S:L719-L723]` | Parse `(M failed)` |
| Staging `.uasset` without `.uexp` | Silently skipped `[S:L794-L801]` | Pre-flight pairing check |
| Treating `verify` as a "game will load it" test | Chunk-hash check only `[S:L438-L503]` | In-game test is still needed |
| Calling `scriptobjects.bin` "required by retoc" | Optional upstream `[S:L807-L811]` | Say "required by project policy" |
| Claiming `to-zen` reads a pre-built `.jmap` or needs `gen-script-objects` | Not part of this pipeline. `gen-script-objects` builds a script-objects *container* from `.jmap` (separate command) | Reuse `scriptobjects.bin` from `to-legacy` |
| Hardcoding/echoing the AES key | Secret; local scripts redact `[LOCAL: Common.ps1:30]` | Env var + redacted errors |
| Extracting the whole game | Unneeded; filters exist `[S:L117]` | Targeted `--filter` |
| Writing output into `~mods/` or the game folder | Unverum wipes `~mods/` on Build (see `unverum-mod-packager`) | Scratch/staging → `dist/` |

---

## 6. Open Hypotheses (NOT facts — do not present as such)

- `[HYPOTHESIS]` A UE 5.1 container built **without** `scriptobjects.bin` fails in-game.
  Upstream treats it as optional. Untested. Policy keeps it required until disproven.
- `[OBSERVATION]` `docs/VERSION_HISTORY.md` records a retoc-built container passing
  `retoc verify`. **Not yet run:** a control `verify` against a vanilla Sparking! ZERO container.
- `[OBSERVATION]` The local pipeline requires an AES key for Sparking! ZERO containers
  `[LOCAL: Extract-RequiredAssets.ps1:16-17]`. Upstream docs make no game-specific statement.
- `[HYPOTHESIS]` Passing a single `.utoc` instead of the `Paks` directory fails to resolve
  dependencies. Upstream accepts both forms. Policy uses the directory.
