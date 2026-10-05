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
4. **File Length Ceiling:**
   * Strictly **300 lines max per file**. Decompose monolithic files into focused, modular domain components before exceeding this limit.
5. **Commit Standards:**
   * Use Conventional Commits 1.0.0 (`feat:`, `fix:`, `chore:`, `docs:`, `test:`).
   * Commit messages must describe the concrete technical change, not opaque sprint steps.
6. **Currency & Formatting Rules:**
   * In chat: Always escape dollar signs (`\$1.00`) or wrap in backticks (`$1.00`).
   * In files & artifacts: Always use clean, unescaped dollar signs (`$1.00`).

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

### 1.2 Mandatory 3-Step Research Cadence
Never generate research documents in a vacuum or run a superficial 30-second query:
1. **Search & Primary Source Identification:** Formulate targeted queries to identify authoritative primary repositories (GitHub, official docs, trusted mod hubs).
2. **Raw Extraction:** Fetch and inspect the raw text/markdown directly (e.g. `raw.githubusercontent.com/.../README.md`) rather than summarizing search result snippets.
3. **Chat Alignment First:** Present the verified facts, the raw receipts, and their architectural implications in chat for human critique before writing any permanent file to `docs/research/`.

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

---

## 2. Mandatory Implementation Planning Gate (Zero Exceptions)

1. **No Unapproved Code Changes:**
   * The agent is **strictly FORBIDDEN** from creating, editing, or deleting ANY source code or configuration files without first authoring an `implementation_plan.md` artifact and receiving the user's explicit, written approval in chat.
2. **No "Quick Fix" Exemptions:**
   * Even for trivial one-line bug fixes, syntax corrections, or minor tweaks, the agent **MUST NOT** touch code without presenting the diagnosis and diff in an implementation plan and waiting for explicit approval.
3. **Strict Scope Creep Invariant:**
   * The agent must never modify parameters outside the diagnosed issue without explicit justification and user sign-off in the plan.

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
