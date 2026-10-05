---
name: utoc-signature-bypass
description: Mandatory when verifying the Dragon Ball Sparking! ZERO UTOC Signature Bypass prerequisite. Enforces two-file layout (dsound.dll + plugins/DBSparkingZeroUTOCBypass.asi), installation path, and the strict non-distribution policy in mod release archives.
---

# Skill: UTOC Signature Bypass

> **Status:** AUDITED & ACTIVE (Phase 5 Passed)  
> **Source receipts:** Unverum repository @ commit [`e41b135`](https://github.com/TekkaGB/Unverum/tree/e41b135e8900187b9a7a06f52bf723f4a7ba35e1)  
> **Reference folder:** [`docs/dependencies/utoc-bypass/`](../../../docs/dependencies/utoc-bypass/README.md)  
> (source receipts: `docs/dependencies/unverum/source-receipts-setup.md`)

Every fact below cites primary source receipts. `[SETUP:Lx]` = line `x` of `Unverum/Setup.cs`. If a claim is not proven by source code, it is categorized as `[HYPOTHESIS]` (see `AGENTS.md` §1.3).

---

## 1. Scope Boundaries

**Use this skill for:**
- Verifying the presence and checksums of the runtime signature bypass in local test environments.
- Enforcing non-distribution boundaries in release packages.
- Understanding the coexistence boundaries between the bypass and `RE-UE4SS`.

**Do NOT use this skill for (hand off instead):**
| Task | Owner skill |
|---|---|
| Packing or verifying `.utoc`/`.ucas` containers | `retoc-iostore-packer` |
| Asset serialization / JSON modification | `uassetgui-asset-serialization` |
| Runtime Lua scripting | `ue4ss-runtime-scripting` |
| Release packaging & Unverum deployment | `unverum-mod-packager` |

---

## 2. Verified Bypass Architecture

### 2.1 The Two-File Requirement
- `[FACT]` The bypass consists of **two files**, not just `dsound.dll` (`docs/dependencies/unverum/source-receipts-setup.md` L98-L108):
  1. `SparkingZERO\Binaries\Win64\dsound.dll`
  2. `SparkingZERO\Binaries\Win64\plugins\DBSparkingZeroUTOCBypass.asi`
- `[FACT]` Target directory: `<GameRoot>\SparkingZERO\Binaries\Win64\` (`[SETUP:L60-L62]`).
- `[FACT]` Unverum checks specifically for `plugins\DBSparkingZeroUTOCBypass.asi` (`[SETUP:L76-L77]`). If missing, it automatically extracts both files from its embedded resources on setup and on each Build click (`[SETUP:L82-L97]`, `MainWindow.xaml.cs` L826).

### 2.2 Verified Checksums
`[LOCAL]` Hashes computed from the files embedded in Unverum @ `e41b135`:

| File | Size (bytes) | SHA256 |
|---|---|---|
| `dsound.dll` | 415,232 | `BB8767F918C52A2AD055D2DE9BAFFD2478598643B9894F09ABD20D1F1FFD170C` |
| `plugins\DBSparkingZeroUTOCBypass.asi` | 43,520 | `40D722790783D2093C21CB1688203E61B3129481E513DF04C893311DD0EC86AA` |

---

## 3. Strict Non-Distribution Policy (`AGENTS.md` §4.1)

- `[POLICY]` Mod release archives (`CompleteStory.zip`) must **NEVER** bundle `dsound.dll` or `DBSparkingZeroUTOCBypass.asi`.
- The bypass is an end-user environment prerequisite installed once by the mod manager (Unverum) or manually by the user. Bundling these files in mod archives creates installation conflicts and violates clean packaging invariants.

---

## 4. Pre-Flight Checklist (run BEFORE in-game testing)

- [ ] Local environment check: Verify `<GameRoot>\SparkingZERO\Binaries\Win64\dsound.dll` exists.
- [ ] Local environment check: Verify `<GameRoot>\SparkingZERO\Binaries\Win64\plugins\DBSparkingZeroUTOCBypass.asi` exists.
- [ ] Release audit: Verify that `dist/` and release archives contain **only** `.pak`, `.utoc`, and `.ucas` containers.

---

## 5. Post-Flight Checklist (run AFTER launching game with mod)

- [ ] Game successfully boots without signature rejection dialogs.
- [ ] Modded container mounted and recognized by engine without crashing on title screen or character select.

---

## 6. Negative Constraints (Hallucination Defense)

| Forbidden | Why (receipt) | Do instead |
|---|---|---|
| Stating bypass is only `dsound.dll` | Unverum embeds and installs both `dsound.dll` and `.asi` | Treat as a 2-file prerequisite |
| Bundling `dsound.dll` in release packages | Violates `AGENTS.md` §4.1; Unverum installs it automatically | Ship only container files |
| Stating technical details of the RSA hook as proven facts | Nexus page returned 403; inner C++ code not audited | Categorize as `[HYPOTHESIS]` |
| Assuming Unverum's `Restart` wipes `dsound.dll` | `dsound.dll` is not in the wipe list (`ModLoader.cs` L55) | Rely on Unverum's `CheckPatch` |

---

## 7. Open Hypotheses (NOT facts — do not present as such)

- `[HYPOTHESIS]` `dsound.dll` functions as an Ultimate ASI Loader proxy that scans `plugins\` and executes `DBSparkingZeroUTOCBypass.asi`. Suggested by naming and layout; not audited from C++ source.
- `[HYPOTHESIS]` In-memory signature hook disables `FIoStoreTocHeader` signature validation. Unproven mechanism hypothesis.
