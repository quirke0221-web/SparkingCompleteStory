# `retoc` CLI & Version Reference

**Pinned:** retoc `0.1.5` · **Audited commit:** [`885a8da`](https://github.com/trumank/retoc/tree/885a8dae740cb1ce1e41ff2e74f67f9f0c118237)
**Receipts:** every statement here is backed by a verbatim excerpt in
[`source-receipts-arguments.md`](source-receipts-arguments.md),
[`source-receipts-commands.md`](source-receipts-commands.md), or
[`source-receipts-behavior.md`](source-receipts-behavior.md).
Line refs `L#` point to `retoc_cli/src/main.rs` at that commit.

> **Note:** the upstream README's `--help` block ([`raw-readme.md`](raw-readme.md)) is
> older than the source. It omits `asset-registry`, and it shows `--aes-key` as required.
> In source, `--aes-key` is optional (`Option<String>`, L281). Trust the source.

---

## 1. Global Options (`Args`, L279-L288)

These go **before** the subcommand: `retoc [GLOBAL OPTIONS] <COMMAND> ...`

| Option | Type | Notes |
|---|---|---|
| `-a, --aes-key <AES_KEY>` | optional string | Parsed as an AES key and stored in the config. |
| `--override-container-header-version <V>` | optional enum | Overrides the header version that `--version` selects. |
| `--override-toc-version <V>` | optional enum | Overrides the TOC version that `--version` selects. |
| `-V, --version` | — | Prints retoc's own version (`#[clap(version)]`). |

The upstream README notes that the same override must be applied when repacking
if one was needed to extract ([`raw-readme.md`](raw-readme.md) "overrides").

---

## 2. Subcommands (`enum Action`, L238-L275)

| Command | Positional args | Options | Upstream doc comment |
|---|---|---|---|
| `manifest` | `<UTOC>` | — | Extract manifest from .utoc |
| `info` | `<PATH>` | — | Show container info |
| `list` | `<UTOC>` | `--all` `--hash` `--package` `--size` `--path` `--store` | List files in .utoc (directory index) |
| `verify` | `<UTOC>` | — | Verify IO Store container |
| `unpack` | `<UTOC> <OUTPUT>` | `-v, --verbose` | Extracts chunks (files) from .utoc |
| `unpack-raw` | `<UTOC> <OUTPUT>` | — | Extracts raw chunks from container |
| `pack-raw` | `<INPUT> <UTOC>` | — | Packs directory of raw chunks into container |
| `to-legacy` | `<INPUT> <OUTPUT>` | see §3 | Converts assets and shaders from Zen to Legacy |
| `to-zen` | `<INPUT> <OUTPUT>` | see §4 | Converts assets and shaders from Legacy to Zen |
| `get` | `<INPUT> <CHUNK_ID> [OUTPUT]` | — | Get chunk; stdout if output is `-` or omitted |
| `dump-test` | `<INPUT> <OUTPUT_DIR> <PACKAGE_ID>` | — | Dump test |
| `gen-script-objects` | `<INPUT.jmap> <OUTPUT.utoc>` | `--version` (required) | Generate script objects global container from `.jmap` |
| `print-script-objects` | `<INPUT.utoc>` | — | Print script objects from container |
| `asset-registry` | `<AssetRegistry.bin>` | — | Parse and print asset registry contents |

`list --all`: "By default only unique chunks will be listed. --all will also list
chunks overriden by patch containers" (L54).

---

## 3. `to-legacy` (`ActionToLegacy`, L107-L152)

```console
retoc [--aes-key <KEY>] to-legacy [OPTIONS] <INPUT> <OUTPUT>
```

| Arg / Option | Type | Behavior (receipt) |
|---|---|---|
| `<INPUT>` | path | ".utoc or directory with multiple .utoc (e.g. Content/Paks/)" (L108) |
| `<OUTPUT>` | path | Writes a `.pak` if the path ends in `.pak`, otherwise a directory (L628) |
| `-f, --filter` | repeatable string | **Substring** match: `package_path.contains(f)` (L692). Not a prefix, glob, or regex |
| `--version` | **optional** `EngineVersion` | "Engine version override" (L137) |
| `--no-assets` | flag | Skip conversion of assets |
| `--no-shaders` | flag | Skip conversion of shader libraries |
| `--no-script-objects` | flag | Skip extraction of script objects |
| `--no-compres-shaders` | flag | Skip compression of shader libraries (upstream spelling) |
| `-d, --dry-run` | flag | No files written |
| `--script-cell` | repeatable | Extra Verse script cells (not relevant to UE 5.1) |
| `-v, --verbose` / `--debug` / `--no-parallel` | flags | Logging / single-threaded |

**Behaviors that matter:**
- If a package fails to convert, it is logged and counted, but the process still
  succeeds (L719-L723). The summary line is `Extracted N (M failed) legacy assets to ...` (L738).
- `scriptobjects.bin` is written to the output root unless `--no-script-objects` is
  set, when the container TOC version is above `PerfectHash` (L673-L677).
- The `../../../` mount prefix is stripped from output paths (L715).

---

## 4. `to-zen` (`ActionToZen`, L155-L184)

```console
retoc to-zen --version <EngineVersion> [OPTIONS] <INPUT> <OUTPUT.utoc>
```

| Arg / Option | Type | Behavior (receipt) |
|---|---|---|
| `<INPUT>` | path | "Input directory or .pak" (L156) |
| `<OUTPUT>` | path | "Output .utoc" (L159). A sibling `.pak` index is also written (L938-L940) |
| `--version` | **required** `EngineVersion` | Selects TOC + container header versions (L169, L773-L775) |
| `-f, --filter` | repeatable string | Substring match on input paths (L784-L786) |
| `--script-cell` | repeatable | Verse only |
| `-v, --verbose` / `--debug` / `--no-parallel` | flags | Logging / single-threaded |

**Behaviors that matter:**
- The mount point is hard-coded to `../../../` (L769). The input tree is therefore
  laid out from the game root (e.g. `SparkingZERO/Content/...`).
- Only `.uasset`/`.umap` files with a sibling `.uexp` are packed. Others are
  **skipped with an info message, not an error** (L794-L801).
- `scriptobjects.bin` is **optional**. If a file with that name is in the input,
  it is parsed "for VNI support and import checking" (L807-L811).
- `.ubulk`, `.uptnl`, `.m.ubulk` companions are picked up if present (L855-L857).

---

## 5. `verify` (L438-L505)

Reads every chunk from `<UTOC>` and its sibling `.ucas` (L440). It compares the first 20
bytes of a BLAKE3 hash of each chunk against the TOC's stored hash (L485-L497).
It prints `verified` on success and fails with `hash mismatch for chunk #N` otherwise.

**It does not** check the directory index, package dependencies, signatures, or
whether a game will mount or load the container.

---

## 6. `EngineVersion` Mappings (`retoc/src/version.rs` L10-L79)

| `EngineVersion` | TOC version | Container header version | UE5 object version |
|---|---|---|---|
| `UE4_25` | `DirectoryIndex` | `Initial` | — |
| `UE4_26` | `DirectoryIndex` | `Initial` | — |
| `UE4_27` | `PartitionSize` | `Initial` | — |
| `UE5_0` | `PerfectHashWithOverflow` | `LocalizedPackages` | `LargeWorldCoordinates` |
| **`UE5_1`** | **`PerfectHashWithOverflow`** | **`OptionalSegmentPackages`** | **`AddSoftObjectPathList`** |
| `UE5_2` | `PerfectHashWithOverflow` | `OptionalSegmentPackages` | `DataResources` |
| `UE5_3` | `PerfectHashWithOverflow` | `NoExportInfo` | `DataResources` |
| `UE5_4` | `OnDemandMetaData` | `NoExportInfo` | `PropertyTagCompleteTypeName` |
| `UE5_5` | `ReplaceIoChunkHashWithIoHash` | `SoftPackageReferences` | `AssetRegistryPackageBuildDependencies` |
| `UE5_6` | `ReplaceIoChunkHashWithIoHash` | `SoftPackageReferencesOffset` | `OsSubObjectShadowSerialization` |
| `UE5_7` | `ReplaceIoChunkHashWithIoHash` | `SoftPackageReferencesOffset` | `ImportTypeHierarchies` |

Values are spelled exactly as declared (`rename_all = "verbatim"`, version.rs L9), e.g. `UE5_1`.
`PerfectHashWithOverflow` sorts after `PerfectHash` in `EIoStoreTocVersion` (`retoc/src/lib.rs`
L1163-L1174), so `UE5_1` containers take the scriptobjects code paths above.
