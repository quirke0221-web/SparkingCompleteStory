# Complete Story: Agent Engineering Guardrails & Rules

## 0. Workspace & Version Control Invariants

1. **Git as the Single Source of Truth (SSOT):**
   * Git is the sole version control system.
   * Creating manual version-suffixed folders, backup trees, or timestamped copies (e.g., `_v2`, `_backup`, `_old`, or manual milestone folders) is **strictly FORBIDDEN**. All versioning, branching, and historical provenance must be managed exclusively through Git commits, branches, and tags.
2. **Anti-Duplication Law (Zero Working Tree Clones):**
   * Every file must have exactly one authoritative location in the repository.
   * Duplicating scripts, source files, or configuration across multiple directories is **strictly FORBIDDEN**.
   * Superseded, experimental, or retired code must be replaced or deleted via Git commits, never copied into parallel "working" and "archive" staging directories in the live tree.
3. **Clean Root Invariant:**
   * The repository root must remain pristine. Only standard project configuration files, `README.md`, `AGENTS.md`, and `.gitignore` may reside in the root.
   * All documentation must reside within `docs/` according to the Documentation Taxonomy.
4. **Code File Length Ceiling:**
   * Strictly **300 lines max per code/script file** (`.ps1`, `.lua`, `.py`, `.c`, etc.). Decompose monolithic implementation files into focused, modular domain components before exceeding this limit.
   * Documentation files (`.md` in `docs/`) are explicitly **exempt** from this ceiling to prevent artificial truncation of specifications, architecture plans, research receipts, and operational runbooks.
5. **Commit Standards & Turn-Boundary Cadence:**
   * Use Conventional Commits 1.0.0 (`feat:`, `fix:`, `chore:`, `docs:`, `test:`) formatted strictly per [`docs/commit.md`](docs/commit.md).
   * **Turn-Boundary Commit Invariant:** An agent must never leave dirty, uncommitted changes in the working tree at the conclusion of a turn. Every completed, verified milestone or bug fix must be committed and pushed to `origin main` before ending the turn, strictly following the operational cadence in [`docs/git-workflow.md`](docs/git-workflow.md).
6. **Currency & Formatting Rules:**
   * In chat: Always escape dollar signs (`\$1.00`) or wrap in backticks (`$1.00`).
   * In files & artifacts: Always use clean, unescaped dollar signs (`$1.00`).
7. **Authoritative Execution Domains:**
   * **Native Build & Packaging CLI:** Exclusively located in `crates/complete-story-cli/` (`cargo run -p complete-story-cli -- build --deploy`). Legacy PowerShell scripts in `helpers/` are superseded and retired.
   * **In-Game UE4SS Mod:** Exclusively located in `CompleteStory/` (with its active Lua entry point at `CompleteStory/scripts/main.lua`). Never treat `CompleteStory/scripts/` as legacy; it is the active UE4SS mod required by the game engine.

---

## 1. First-Principles Research & Anti-Hallucination Protocol

When conducting research, investigating dependencies, analyzing game systems, or proposing technical solutions, any agent **MUST ALWAYS** obey the following laws:

### 1.1 The "Receipts-First" Investigation Invariant
* An agent is **strictly FORBIDDEN** from stating technical facts, tool capabilities, command-line arguments, file formats, or engine mechanics based purely on parametric memory (pre-training weight completion).
* Every technical assertion made in chat or documented in `docs/research/` must be backed by an **Empirical Receipt**:
  1. **Primary Source Receipt:** A direct live URL to an official repository, documentation page, or release tag fetched live during the session.
  2. **Verbatim Evidence:** An unedited excerpt from the raw README, source file, or CLI help text.
  3. **Local Confirmation:** A reference to an existing verified script, test run, or local configuration.
* If an agent cannot obtain a live receipt, it is barred from presenting the claim as fact. It must explicitly state:  
  `[UNVERIFIED HYPOTHESIS: Requires investigation via <specific search/tool>]`

### 1.2 Mandatory 4-Step Research Cadence (The Strategy-First Pipeline)
Never jump blindly into research or generate research documents in a vacuum:
1. **Pre-Research Strategy & Matrix Artifact:** An agent is **strictly FORBIDDEN** from beginning research without first authoring a dedicated **Research Strategy Artifact** (stored in the session brain artifacts directory, never committed to git). This artifact must establish:
   - A comprehensive **Target Component Matrix** cataloging every function, RVA, asset, struct, or dependency involved, detailing: (a) Target identifier/file, (b) Assigned investigation track, (c) **Why** it must be researched, and (d) **What** it's for in the game engine.
   - Explicit, **falsifiable hypotheses** with defined test and failure conditions.
   - The execution sequence connecting the investigation to the forthcoming research document and implementation plan.
2. **Targeted Dissection & Evidence Capture:** Execute the assigned investigation track (Native C++, Asset Schema, or Runtime Reflection) against local binaries, tools, and raw sources.
3. **Chat Alignment First:** Present verified facts, raw receipts, and architectural implications in chat for human critique before committing any permanent file to `docs/research/`.
4. **Permanent Research Record:** Synthesize approved findings into a numbered research document under `docs/research/<subsystem>/` following the 06 Protocol.

### 1.3 Strict Epistemic Labeling (Anti-Assumption Creep)
To prevent unverified guesses from mutating into requirements, all project documentation and research notes must categorize claims using explicit epistemic tags:
* `[FACT]`: Formally proven by live documentation, official source code, or a repeatable local test.
* `[OBSERVATION]`: What was directly observed in runtime execution, logs, or UI (without assuming the root cause).
* `[HYPOTHESIS]`: An unproven theory that requires a specific verification test before code can be written for it.
* `[DISPROVEN]`: A theory that was tested and failed.

### 1.4 The Skill-Gated Dependency Law (Zero Unvetted APIs)
An agent is **strictly FORBIDDEN** from proposing or writing implementation code that integrates, calls, or executes any external tool, CLI binary, library, or engine framework unless:
1. **Approved in Matrix:** The dependency is formally approved, pinned, and audited in `docs/dependencies/dependencies.md`.
2. **Reference Material Stored:** Primary source documentation and CLI references are preserved in its dedicated subfolder under `docs/dependencies/<dependency>/`.
3. **Agent Skill Exists:** A dedicated, research-backed **Agent Skill** exists providing verified usage patterns and anti-hallucination notes.
4. **No Vibe-Coded Skills:** Skills cannot be hallucinated from parametric memory; they must be authored strictly from the verified reference materials.

### 1.5 The Pre-Implementation Forensic Research Gate (The "06 Protocol")
* **The 4-Tier Governance Pipeline:** Every technical milestone must progress strictly through four distinct gates:
  $$\text{Strategy Artifact (Brain)} \longrightarrow \text{Research Document (docs/research/)} \longrightarrow \text{Implementation Plan (Brain)} \longrightarrow \text{Code Implementation}$$
* **Hard Implementation Plan Gate:** An agent is **strictly FORBIDDEN** from authoring an Implementation Plan or modifying code for any feature, engine hook, data structure, or native mechanic until a dedicated, receipt-backed research document (modeled after the depth of [`docs/research/episode-battle-subsystem/06-isplayable-dissection-and-mechanics.md`](docs/research/episode-battle-subsystem/06-isplayable-dissection-and-mechanics.md)) exists and has been reviewed. Drafted implementation plans without a prior approved research document are deemed invalid and hallucination-prone.
* **Pre-Research Strategy Prerequisite:** An agent is **strictly FORBIDDEN** from starting research without first authoring the Research Strategy Artifact mandated in §1.2.
* **Dependency-Aware Investigation Tracks:** While `IsPlayable` required a 5-step machine code disassembly protocol, different subsystems require different investigative strategies aligned with our approved dependency matrix:
  1. **Native C++ & Memory Engine Track** (`Dumper-7`, `MinHook`):
     - *Step 1 (Symbol & Address Resolution):* Resolve exact function RVA, virtual method offset, or memory signature.
     - *Step 2 (Instruction & Register Mapping):* Disassemble machine instructions, identifying calling conventions and register usage (`rcx`, `rdx`, `rax`, stack frames).
     - *Step 3 (Control Flow & Struct Alignment):* Trace sub-calls, pointer offsets, bitfield layouts, and data structures (e.g. `TMap`, `FString`, `FName`).
     - *Step 4 (Global Call Site Analysis):* Scan the PE image (`.text`) for all callers to uncover hidden callers, UI widget builders, or Blueprint bypasses.
     - *Step 5 (Structural Documentation):* Author a dedicated research document with full disassembly listings, opcodes, and control flow diagrams.
  2. **Asset Serialization & Schema Track** (`FModel` + `.usmap`, `UAssetGUI`):
     - *Step 1 (Schema & Type Reflection):* Use `.usmap` mappings to resolve unversioned property indices, struct types, and property names.
     - *Step 2 (AST & Binary Layout Dissection):* Inspect raw JSON AST and decode binary export payloads (e.g., base64 `RawExport`), mapping byte offsets, string lengths, and flags.
     - *Step 3 (Container & Packaging Validation):* Trace Zen IoStore container requirements (`retoc`) and dependency tables (`CreateBeforeCreateDependencies`).
     - *Step 4 (Structural Documentation):* Author a dedicated research document detailing exact field offsets, schemas, and serialization rules.
  3. **Runtime Reflection & Scripting Track** (`RE-UE4SS`):
     - *Step 1 (UObject / UFunction Verification):* Confirm class hierarchy, reflection properties, and native function availability in memory.
     - *Step 2 (Frame & Parameter Inspection):* Trace `FFrame` parameter processing and return value handling (adhering to parameterless hook limitations).
     - *Step 3 (Memory Safety & Lifecycle Audit):* Verify GameThread execution safety and ban transient Slate widget scraping.
     - *Step 4 (Structural Documentation):* Author a dedicated research document detailing runtime hooks, verified object paths, and memory safety boundaries.

---

## 2. Intent Alignment & Non-Technical Playtester Gate

1. **User Persona as Creative Director & Playtester:**
   * The user guides project features, character story beats, and game balance, and validates behavior by playtesting in-game. The agent is responsible for technical execution, syntax, tool orchestration, and Git hygiene.
   * Agents are **strictly FORBIDDEN** from demanding raw code diff reviews or asking intimidating technical programming questions. Communication must remain conversational, practical, and focused on player-facing outcomes.
2. **Plain-English Intent Alignment (Zero Rogue Modifications):**
   * Before modifying source code, game assets, or configuration, the agent must present a clear, conversational plan explaining:
     - What feature or fix is being built.
     - Which game components or files are affected.
     - What the user will be able to test in Sparking! ZERO once completed.
   * The agent must wait for the user's conversational sign-off (e.g. "Looks good", "Go ahead", "Let's do it") before editing code.
3. **Strict Scope Creep Invariant:**
   * The agent must never modify game parameters, assets, or logic outside the agreed intent.
4. **Verifiable In-Game Delivery:**
   * Upon completing any milestone or bug fix, the agent must build and deploy the changes via `crates/complete-story-cli` (`cargo run -p complete-story-cli -- build --deploy`) and provide simple, step-by-step instructions for how the user can test the change in-game.

---

## 3. Documentation Taxonomy & Lifecycle

Documentation must follow a strict hierarchical dependency order:

```text
docs/
├── PRD.md                    <-- Master Product Requirements (Single Source of Truth)
├── ARCHITECTURE.md           <-- System Architecture (Authored ONLY after PRD approval)
├── dependencies/             <-- Master Dependency Matrix & Reference Folders
│   ├── dependencies.md       <-- Pinned matrix with skill status & anti-hallucination notes
│   └── <dependency>/         <-- Primary source references, raw READMEs & CLI docs
├── ADRs/                     <-- Architectural Decision Records (Formal decisions only)
├── roadmaps/                 <-- Milestone execution plans
└── research/                 <-- Grounded investigations with verified receipts
```

1. **Requirements Before Architecture:** `docs/ARCHITECTURE.md` is strictly informed by `docs/PRD.md`. It must never be drafted or updated ahead of approved product requirements.
2. **No Premature ADRs:** Never create Architectural Decision Records based on vibe-coded experiments or unvetted assumptions. ADRs document settled, verified decisions made collaboratively with the user.
3. **Research First:** Any new feature, third-party dependency, or engine hook must have a corresponding receipt-backed document in `docs/research/` before architecture or planning begins.

---

## 4. Game Modding & Engine Invariants

1. **Unreal Engine 5.1.1 Zen/IoStore Compliance:**
   * Asset modifications must be packaged cleanly into standard UE5 IoStore containers (`.pak`, `.utoc`, `.ucas`) using verified community tools (`retoc`).
   * Containers must be compatible with the community **UTOC Signature Bypass** and mount cleanly via `SparkingZERO/Content/Paks/~mods/` or Unverum.
   * Mod release packages must never bundle third-party bypass or injection binaries.
2. **Runtime Memory Safety (Zero Slate Scraping):**
   * Slate and UMG UI widgets in Unreal Engine are pooled and transient. Agents are **strictly FORBIDDEN** from writing runtime scripts that poll, screen-scrape, or reflect Slate widgets across frames (e.g. `FindAllOf("TextBlock")`), as this causes fatal native memory access violations.
   * State and playability must be managed through stable engine data structures or native lifecycle hooks.
3. **Non-Destructive Coexistence Invariant:**
   * Adding custom content must never overwrite, mutate, or break the 12 original stock character campaigns or the player's vanilla save data (`MainGameSaveData`).
