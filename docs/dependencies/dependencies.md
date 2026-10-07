# Dependency Matrix & Ecosystem Protocol

## 1. Purpose & Core Philosophy

Sparking ZERO: Complete Story leverages battle-tested, actively maintained open-source modding tools and Unreal Engine 5 frameworks to avoid "reinventing the wheel."

Every dependency introduced must:
1. Be documented in this matrix with its exact pinned version, target layer, and architectural rationale.
2. Have its primary source reference documentation preserved in its dedicated subfolder under `docs/dependencies/<dependency>/`.
3. Include an **Agent Skill Status** audit to prevent AI coding assistants from hallucinating obsolete APIs, incorrect CLI flags, or invalid engine methods.
4. **The Skill-Gated Law:** No agent may write implementation code using a dependency until a dedicated, research-backed Agent Skill is established.

---

## 2. Dependency Ingestion Protocol

Whenever a developer or agent proposes introducing a new tool, binary, or framework:

1. **Check for Existing Ecosystem Overlap:** Verify that an existing tool in the toolchain cannot already solve the problem.
2. **Audit Tool Health & Primary Receipts:**
   * Must have an active repository or verified community release.
   * Must fetch the raw documentation or README into `docs/dependencies/<dependency>/`.
3. **Agent Skill & Hallucination Check:**
   * Formulate the **Hallucination Prevention Notes** documenting known agent failure modes.
   * Author a dedicated Agent Skill before any implementation code touches the tool.
4. **Update This Matrix:** Add an entry to the matrix below with version, rationale, reference folder, and skill status.

---

## 3. Modding Toolchain & Dependency Matrix

| Tool / Dependency | Pinned Version (audited) | Source | Purpose | Reference Folder | Receipts Audit | Agent Skill Status | Hallucination Prevention Notes |
|---|---|---|---|---|---|---|---|
| **`retoc`** | `0.1.5` @ `885a8da` | [`trumank/retoc`](https://github.com/trumank/retoc) | Extract and pack UE5 IoStore containers (`to-legacy`, `to-zen`, `verify`). | [`retoc/`](retoc/README.md) | Done | Active Skill (Audited) | `--aes-key` goes before the subcommand. `--version UE5_1` is required for `to-zen`. `scriptobjects.bin` is optional upstream, required by project policy. `verify` only checks chunk hashes. |
| **`UAssetGUI` / `UAssetAPI`** | `1.1.0` @ `4855f8c` / `bee2f9b` | [`atenfyr/UAssetGUI`](https://github.com/atenfyr/UAssetGUI) | Convert `.uasset` to JSON and back (`tojson`, `fromjson`). | [`uassetgui/`](uassetgui/README.md) | Done | Active Skill (Audited) | `tojson` requires `VER_UE5_1` (dotted versions fail). Mapping argument must be bare name in `Data\Mappings\` (full paths fail). `fromjson` auto-writes companion `.uexp`. |
| **`RE-UE4SS`** | `3.0.1 Beta` @ `4e5461c` | [`UE4SS-RE/RE-UE4SS`](https://github.com/UE4SS-RE/RE-UE4SS) | Runtime injection, Lua scripting, Live GUI Inspector, and CXX Header generation. | [`ue4ss/`](ue4ss/README.md) | Done | Active Skill (Audited) | `docs.ue4ss.com/dev` is 4.x; use only the pinned copies. Class path is `/Script/SS.SSDragonAdventureIFCSManager`. Live GUI enabled via `GuiConsoleVisible = 1`. No widget searching/reflection in hooks (`AGENTS.md` §4.2). |
| **`FModel`** | `4.4.4.0` | [`4sval/FModel`](https://github.com/4sval/FModel) | Visual Unreal Engine 5 IoStore archive explorer and data property inspector. | [`fmodel/`](fmodel/README.md) | Done | Active Skill (Audited) | Requires `.usmap` mapping file (`SparkingZERO.usmap`) and AES key. Operates on `GAME_UE5_1`. Never commit raw unpacked asset dumps to Git. |
| **`Dumper-7`** | main @ `Encryqed/Dumper-7` | [`Encryqed/Dumper-7`](https://github.com/Encryqed/Dumper-7) | Automated C++ SDK generator for memory structures, member offsets, and function signatures. | [`dumper-7/`](dumper-7/README.md) | Done | Active Skill (Audited) | Emits compilable `.hpp` C++ headers into `.tools/SDK/`. Handles bitfields and alignments. Consult headers instead of pattern-scanning raw assembly. |
| **`UTOC Signature Bypass`** | Files embedded in Unverum @ `e41b135` (SHA256 in folder) | Unverum source (Nexus `mods/18` returned 403) | Lets the game load modded containers. Two files: `dsound.dll` + `plugins\DBSparkingZeroUTOCBypass.asi` in `SparkingZERO\Binaries\Win64`. | [`utoc-bypass/`](utoc-bypass/README.md) | Done (mechanism is `[HYPOTHESIS]`) | Active Skill (Audited) | It is two files, not just `dsound.dll`. How it works is unverified. Our release must not bundle it. |
| **`Unverum`** | commit `e41b135` | [`TekkaGB/Unverum`](https://github.com/TekkaGB/Unverum) | Mod manager users install our containers with. | [`unverum/`](unverum/README.md) | Done | Active Skill (Audited) | Supports Sparking! ZERO in source (README is outdated). Appends `_9_P` itself. Needs a `.pak` to pick up `.utoc`/`.ucas`. Every Build deletes `~mods` and UE4SS files. |
| **`Rust Compiler Baseline`** | `rustc 1.98.1` / `cargo 1.98.1` | [`rust-lang/rust`](https://github.com/rust-lang/rust) | Native CLI build environment and compiler. | [`rust-toolchain/`](rust-toolchain/README.md) | Done | Active Skill (Audited) | Enforce `<= 300` lines per file. Never hardcode AES key in code. Read from `SPARKING_ZERO_AES_KEY`. |
| **`clap`** | `4.6.7` | [`clap-rs/clap`](https://github.com/clap-rs/clap) | Declarative CLI argument parsing and subcommand dispatch. | [`clap/`](clap/README.md) | Done | Active Skill (Audited) | Use `#[derive(Parser, Subcommand)]` and `#[command(subcommand)]`. Requires `features = ["derive"]`. |
| **`serde_json` / `serde`** | `1.0.151` / `1.0.229` | [`serde-rs/json`](https://github.com/serde-rs/json) | AST parsing, array splicing, and deterministic JSON formatting. | [`serde-json/`](serde-json/README.md) | Done | Active Skill (Audited) | Requires `features = ["preserve_order"]`. Mutate `Value::Array` directly via `as_array_mut()`. |
| **`anyhow`** | `1.0.104` | [`dtolnay/anyhow`](https://github.com/dtolnay/anyhow) | Error context chaining and subprocess diagnostics. | [`anyhow/`](anyhow/README.md) | Done | Active Skill (Audited) | Use `.with_context(|| ...)`. Never use raw `.unwrap()` or swallow child process errors. |
| **`zip`** | `8.6.0` | [`zip-rs/zip2`](https://github.com/zip-rs/zip2) | Packaging standalone `.zip` distribution bundles. | [`zip/`](zip/README.md) | Done | Active Skill (Audited) | Use `zip::write::SimpleFileOptions::default()`. Avoid unstable `9.0.0-pre3`. Normalize paths to forward slashes. |
| **`minhook`** | `0.9.0` | [`TsudaKageyu/minhook`](https://github.com/TsudaKageyu/minhook) / [`rust-minhook`](https://crates.io/crates/minhook) | Native x86/x64 API and function detour hooking in `complete-story-runtime`. | [`minhook/`](minhook/README.md) | Done | Active Skill (Audited) | Call `MinHook::create_hook` and `MinHook::enable_all_hooks`. Never hook from unaligned or non-executable addresses. Original trampoline must be cast via `std::mem::transmute`. |
| **`windows-sys`** | `0.59.0` | [`microsoft/windows-rs`](https://github.com/microsoft/windows-rs) | Zero-overhead, strongly typed Win32 system call bindings. | [`windows-sys/`](windows-sys/README.md) | Done | Active Skill (Audited) | Feature-gated imports (`Win32_Foundation`, `Win32_System_LibraryLoader`, etc.). Use raw handles `HMODULE` and `c_void`. |

---

## 4. Agent Skill Roadmap (Hallucination Defense Plan)

Skills live in `.agents/skills/<name>/SKILL.md` and are compiled only from the receipts in the folders above.
Order and status are tracked in the implementation plan.

```text
.agents/skills/
├── sparking-zero-mod-pipeline/SKILL.md      # Tier 1 orchestrator (AUDITED & ACTIVE)
├── rust-pipeline-builder/SKILL.md           # Tier 2 build toolchain (AUDITED & ACTIVE)
├── minhook-native-hooking/SKILL.md          # Tier 2 native detour hooking (AUDITED & ACTIVE)
├── clap-cli-parser/SKILL.md                 # Tier 2 CLI parser (AUDITED & ACTIVE)
├── serde-json-ast/SKILL.md                  # Tier 2 JSON AST (AUDITED & ACTIVE)
├── anyhow-error-handling/SKILL.md           # Tier 2 error handling (AUDITED & ACTIVE)
├── zip-archive-packager/SKILL.md            # Tier 2 zip packager (AUDITED & ACTIVE)
├── retoc-iostore-packer/SKILL.md            # Tier 2 (AUDITED & ACTIVE)
├── uassetgui-asset-serialization/SKILL.md   # Tier 2 (AUDITED & ACTIVE)
├── ue4ss-runtime-scripting/SKILL.md         # Tier 2 (AUDITED & ACTIVE)
├── fmodel-asset-explorer/SKILL.md           # Tier 2 (AUDITED & ACTIVE)
├── dumper-7-sdk-generator/SKILL.md          # Tier 2 (AUDITED & ACTIVE)
├── unverum-mod-packager/SKILL.md            # Tier 2 (AUDITED & ACTIVE)
└── utoc-signature-bypass/SKILL.md           # Tier 2 (AUDITED & ACTIVE)
```