---
name: unverum-mod-packager
description: Mandatory before packaging, naming, or staging Dragon Ball Sparking! ZERO mod release artifacts for Unverum and ~mods deployment. Enforces .pak presence, priority renaming behavior, and protection against Unverum's build-time ~mods wipe.
---

# Skill: Unverum Mod Packager

> **Status:** AUDITED & ACTIVE (Phase 4 Passed)  
> **Pinned version:** Unverum audited at commit [`e41b135`](https://github.com/TekkaGB/Unverum/tree/e41b135e8900187b9a7a06f52bf723f4a7ba35e1)  
> **Reference folder:** [`docs/dependencies/unverum/`](../../../docs/dependencies/unverum/README.md)  
> (verbatim source receipts: `source-receipts-setup.md`, `source-receipts-build.md`, `bundled-ue4ss-settings-ini.md`)

Every rule below cites its source receipt. `[SETUP:Lx]` = line `x` of `Unverum/Setup.cs`, `[BUILD:Lx]` = line `x` of `Unverum/ModLoader.cs`, `[WIN:Lx]` = line `x` of `Unverum/UI/MainWindow.xaml.cs`. If a behavior is not cited here, it is not verified. Do not invent it (see `AGENTS.md` §1).

---

## 1. Scope Boundaries

**Use this skill for:**
- Staging container files (`.pak`, `.utoc`, `.ucas`) for Unverum mod management and manual `~mods/` installation.
- Designing mod release ZIP archives and directory layouts.
- Guarding against Unverum's destructive build-time wipe behavior.

**Do NOT use this skill for (hand off instead):**
| Task | Owner skill |
|---|---|
| Compiling raw containers with retoc (`to-zen`) | `retoc-iostore-packer` |
| Serializing or editing `.uasset` JSON | `uassetgui-asset-serialization` |
| Authoring Lua hooks in `Win64\Mods` | `ue4ss-runtime-scripting` |
| Signature bypass verification | `utoc-signature-bypass` |

---

## 2. Unverum Engine Invariants & Staging Rules

### 2.1 The Companion `.pak` Invariant (CopyFolder Gate)
- `[FACT]` In `ModLoader.CopyFolder` (`[BUILD:L81-L88]`), Unverum iterates over files searching **strictly** for `.pak` files:
  ```csharp
  if (Path.GetExtension(path).Equals(".pak", ...) && paks.ContainsKey(path) && paks[path])
  ```
- `[FACT]` Unverum checks for and copies `.utoc` and `.ucas` **only if a matching `.pak` file exists** in the same directory (`[BUILD:L110-L116]`):
  ```csharp
  if (File.Exists(Path.ChangeExtension(path, ".utoc")) && File.Exists(Path.ChangeExtension(path, ".ucas")))
  ```
- **Rule:** Never distribute or stage `.utoc` and `.ucas` without their matching sibling `.pak`. Unverum will completely ignore them. (Note: `retoc to-zen` automatically generates this companion `.pak`).

### 2.2 Automatic `_9_P` Renaming
- `[FACT]` When Unverum copies mod packages into the game's `~mods/` directory, it automatically appends `_9_P` to the filename (`[BUILD:L90, L114-L115]`):
  - `<name>.pak` -> `<name>_9_P.pak`
  - `<name>.utoc` -> `<name>_9_P.utoc`
  - `<name>.ucas` -> `<name>_9_P.ucas`
- If an author names their file `CompleteStory_P.pak`, Unverum copies it as `CompleteStory_P_9_P.pak`.
- `[FACT]` Unverum isolates each enabled mod into its own alphabetical priority folder (`~mods\a\<modname>\...`, `~mods\b\...`) (`[BUILD:L260-L261, L275-L280]`).

### 2.3 The Destructive Build Wipe Hazard
- `[FACT]` Clicking "Build" in Unverum invokes `ModLoader.Restart` **before** compiling the mod list (`[WIN:L827, L844]`).
- `[FACT]` `Restart` unconditionally deletes the entire `~mods` directory (`[BUILD:L24-L25]`):
  ```csharp
  Directory.Delete(path, true);
  Directory.CreateDirectory(path);
  ```
  It also deletes `SparkingZERO\Mods` (SZModLib) and UE4SS binaries in `Win64` (`[BUILD:L27-L32, L51-L65]`).
- **Rule:** **NEVER** use `~mods/` as a working directory or build output target. All build pipelines must output to an isolated staging directory (e.g. `dist/` or `staging/`).

---

## 3. Release Packaging & Prerequisite Isolation

### 3.1 Prerequisite Isolation (`AGENTS.md` §4.1)
- `[POLICY]` Release packages (`CompleteStory.zip`) must **NEVER** bundle `dsound.dll` or `plugins\DBSparkingZeroUTOCBypass.asi`.
- Unverum automatically checks and installs these bypass files into `SparkingZERO\Binaries\Win64\` during setup and on every Build click (`[SETUP:L28, L76-L97]`, `[WIN:L826]`). Bundling them in mod releases risks overwriting user configurations or conflicting with other ASI plugins.

---

## 4. Pre-Flight Checklist (run BEFORE packaging)

- [ ] Container trio verification: All three files exist with matching basenames (`<name>.pak`, `<name>.utoc`, `<name>.ucas`).
- [ ] Container verification: `retoc verify <name>.utoc` reports `verified`.
- [ ] Staging isolation: Working files are in `dist/` or `staging/`, **never** directly inside the game's `~mods/` folder.
- [ ] Archive audit: Ensure no third-party binaries (`dsound.dll`, `ue4ss.dll`, `.asi`) are present in the package.

---

## 5. Post-Flight Checklist (run AFTER packaging)

- [ ] Unpack or inspect release archive to verify standard structure.
- [ ] Verify file sizes of `.pak`, `.utoc`, `.ucas` are non-zero.
- [ ] If testing with Unverum, confirm that after clicking "Build", files appear under `SparkingZERO\Content\Paks\~mods\<letter>\<modname>\` with `_9_P` extensions.

---

## 6. Negative Constraints (Hallucination Defense)

| Forbidden | Why (receipt) | Do instead |
|---|---|---|
| Packaging `.utoc`/`.ucas` without `.pak` | Unverum `CopyFolder` skips directory if no `.pak` `[BUILD:L87-L116]` | Always include companion `.pak` |
| Outputting build artifacts to `~mods/` | Unverum `Restart` deletes `~mods/` on build `[BUILD:L24-L25]` | Output to `dist/` |
| Bundling `dsound.dll` or `.asi` in release | Violates `AGENTS.md` §4.1; Unverum installs them automatically | Keep release clean of bypass DLLs |
| Storing manual UE4SS mods in `Win64\Mods` | Unverum deletes `Win64\Mods` on build `[BUILD:L62-L63]` | Use mod folder ending in `ue4ss` |
| Hardcoding double `_9_P` suffixes | Unverum appends `_9_P` automatically `[BUILD:L90]` | Use clean base name |

---

## 7. Open Hypotheses (NOT facts — do not present as such)

- `[HYPOTHESIS]` Expected root folder layout inside Unverum-managed release zip archives (e.g. whether users drop zip directly or requires top-level `SparkingZERO/Content/Paks/~mods/`). Not explicitly enforced by Unverum's zip extractor code.
