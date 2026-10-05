# `UTOC Signature Bypass` Dependency Knowledge Base

* **What it is:** Two files that let the game load modded IoStore containers: `dsound.dll` and
  `plugins\DBSparkingZeroUTOCBypass.asi`, installed into `SparkingZERO\Binaries\Win64`.
* **Best available source:** Unverum source @ [`e41b135`](https://github.com/TekkaGB/Unverum/tree/e41b135e8900187b9a7a06f52bf723f4a7ba35e1),
  which embeds and installs both files. See [`../unverum/source-receipts-setup.md`](../unverum/source-receipts-setup.md).
* **Not available:** The Nexus page (`nexusmods.com/dragonballsparkingzero/mods/18`) returned HTTP 403 to our fetcher.
  Search-engine summaries are not primary sources and are not used here.
* **Agent Skill:** `.agents/skills/utoc-signature-bypass/` (not yet written; Phase 5)

Labels: `[FACT]` = source receipt, `[LOCAL]` = measured in this project, `[POLICY]` = project rule,
`[HYPOTHESIS]` = unproven.

---

## 1. Verified Facts

- `[FACT]` Unverum embeds exactly two Sparking! ZERO patch files: `Patch\dsound.dll` and
  `Patch\plugins\DBSparkingZeroUTOCBypass.asi` (`Unverum.csproj` L53-L54).
- `[FACT]` Install path is `<folder of SparkingZERO.exe>\SparkingZERO\Binaries\Win64` (`Setup.cs` L60-L62).
- `[FACT]` Unverum installs them when `plugins\DBSparkingZeroUTOCBypass.asi` is missing there (`Setup.cs` L76-L77).
- `[FACT]` This check lives in `Setup.CheckPatch` (`Setup.cs` L28). It runs at the end of game setup
  (`Setup.cs` L144-L146) and each time the Build button is pressed (`MainWindow.xaml.cs` L826).
- `[LOCAL]` SHA256 of the embedded files at `e41b135` (computed with `Get-FileHash` on the clone):

| File | Size (bytes) | SHA256 |
|---|---|---|
| `dsound.dll` | 415232 | `BB8767F918C52A2AD055D2DE9BAFFD2478598643B9894F09ABD20D1F1FFD170C` |
| `plugins\DBSparkingZeroUTOCBypass.asi` | 43520 | `40D722790783D2093C21CB1688203E61B3129481E513DF04C893311DD0EC86AA` |

---

## 2. Unverified (do not present as fact)

- `[HYPOTHESIS]` `dsound.dll` is an ASI loader that loads `.asi` files from `plugins\`. Suggested by the file
  layout only. Not confirmed from a primary source.
- `[HYPOTHESIS]` The `.asi` disables the IoStore container signature check. Suggested by its name only.
- `[HYPOTHESIS]` Without the bypass, the game rejects or crashes on modded containers. Not tested in this project.
- `[HYPOTHESIS]` It coexists with UE4SS without conflict. Not tested. Note that Unverum's `Restart` does
  **not** delete `dsound.dll` (it is not in the UE4SS delete list, `ModLoader.cs` L55 in the build receipts).
- `[HYPOTHESIS]` The shipping executable is named `SparkingZERO-Win64-Shipping.exe`. Not verified.

---

## 3. Project Rules

- `[POLICY]` Our release must **not** bundle or overwrite `dsound.dll` or the `.asi`. Users get them from
  Unverum or the original author. Our release ships only our `.pak`/`.utoc`/`.ucas`.

---

## Correction Log

**2026-10-05:** The first draft described a signature-verification mechanism (`FIoStoreTocHeader` signature block,
proxy DLL forcing a check to return `true`, forwarding to `System32\dsound.dll`, UE4SS proxy names) with **no
primary source**. Those statements were removed or moved to §2 as `[HYPOTHESIS]`. The missing `.asi` file was added.

---

## Agent Skill Status
* **Status:** Not started. Phase 5 of the implementation plan.
