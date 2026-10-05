# Complete Story: Git Commit Standards & Governance Protocol

## 1. Guiding Principles

1. **Git as Single Source of Truth (SSOT):** Git history is the permanent, authoritative record of the project. Every commit must be atomic, buildable, and self-documenting.
2. **Conventional Commits 1.0.0:** All commits must adhere strictly to the Conventional Commits specification.
3. **Domain Separation Invariant:** Commits must never co-mingle unrelated architectural domains (e.g., do not mix agent skills with game runtime code, or documentation with build scripts).
4. **Concrete Technical Changes Only:** Commits must describe the exact technical change or artifact introduced—never vague phase numbers, sprint names, or task milestones.

---

## 2. Commit Message Structure

Every commit message consists of a **Header**, an optional **Body**, and an optional **Footer**:

```text
<type>(<scope>): <short imperative description>

[optional body: detailed bullet points explaining why and what changed]

[optional footer: breaking changes or references]
```

### 2.1 Commit Header Format
* **Type:** Mandatory. Must be one of the approved types in §3.
* **Scope:** Mandatory. Must identify the architectural domain affected (see §4).
* **Description:** Mandatory.
  * Use the imperative, present tense ("add", "refactor", "enforce", NOT "added", "refactored", "enforcing").
  * Do not capitalize the first letter.
  * Do not place a period (`.`) at the end.
  * Max 72 characters in length.

### 2.2 Commit Body Guidelines
* Separate the body from the header with a single blank line.
* Use bullet points (`- `) to explain:
  1. **What** concrete files or subsystems were modified.
  2. **Why** the change was necessary (motivation, architectural decision).
  3. Any receipts, upstream references, or ADR citations.
* Wrap lines at 72–80 characters for clean terminal rendering.

---

## 3. Approved Types

| Type | When to Use | Example |
| :--- | :--- | :--- |
| `feat` | New feature, capability, or executable agent skill | `feat(skills): implement retoc packaging skill` |
| `fix` | Bug fix in game mod, build script, or runtime code | `fix(runtime): resolve null pointer in character hook` |
| `docs` | Documentation changes (PRD, research, dependencies, ADRs) | `docs(prd): define Goku Mini non-destructive campaign` |
| `refactor` | Code restructuring without changing runtime behavior | `refactor(pipeline): separate asset transform from build` |
| `chore` | Maintenance, repo rules, governance, or `.gitignore` | `chore(governance): update AGENTS.md rules` |
| `test` | Adding or updating syntax parsers, unit checks, or audits | `test(pipeline): add PowerShell AST syntax validation` |
| `perf` | Performance optimization in build pipeline or packing | `perf(retoc): cache IoStore extract filters` |

---

## 4. Scope Taxonomy

To ensure consistency across human engineers and AI agents (Codex, Antigravity), use strictly the following scopes:

* `(governance)`: Agent rules (`AGENTS.md`), commit standards (`docs/commit.md`), and workflow runbooks (`docs/git-workflow.md`).
* `(skills)`: Autonomous agent skills (`.agents/skills/*`).
* `(deps)`: Dependency matrix and primary source reference vault (`docs/dependencies/*`).
* `(prd)`: Product requirements and scope specifications (`docs/PRD.md`).
* `(adr)`: Architectural Decision Records (`docs/ADRs/*`).
* `(research)`: Empirical ecosystem and legacy codebase investigations (`docs/research/*`).
* `(pipeline)`: Master build orchestrator and packaging scripts (`helpers/Build-*.ps1`).
* `(assets)`: Asset transformation and JSON manipulation logic (`helpers/Transform-*.ps1`).
* `(runtime)`: RE-UE4SS Lua scripts and game engine hooks (`CompleteStory/scripts/*`).
* `(helpers)`: Development, staging, and deployment helper utilities (`helpers/*`).
* `(config)`: Project-level settings and path manifests (`config/*`).

---

## 5. Prohibited Commit Patterns (Strict Governance)

The following anti-patterns are **strictly FORBIDDEN** by repository governance:

1. **NO Opaque Sprint or Phase Milestone Names:**
   * ❌ `feat: phase 1 complete`
   * ❌ `chore: milestone 2 cleanup`
   * ❌ `chore: step 3 done`
   * ✅ `feat(skills): implement two-tier autonomous agent skills harness`
   * ✅ `refactor(pipeline): consolidate 12 legacy scripts into 4 modular tools`
2. **NO Vague or Lazy Descriptions:**
   * ❌ `fix: bug fix`
   * ❌ `chore: update files`
   * ❌ `feat: misc improvements`
   * ❌ `wip`
   * ✅ `fix(assets): correct culture-invariant display name for character 0000_00`
3. **NO Cross-Domain Co-Mingling:**
   * ❌ Staging documentation updates together with runtime Lua scripts in one commit.
   * ❌ Staging third-party reference files together with custom mod asset code.
   * ✅ Keep every commit focused on a single, isolated architectural domain.

---

## 6. Pre-Commit Checklist

Before executing `git commit`, verify:
- [ ] Working tree is clean of temporary, untracked, or backup files (`git status`).
- [ ] No files exceed the **300-line ceiling** (`AGENTS.md` §0.4).
- [ ] No secrets, AES keys, or personal paths are staged.
- [ ] PowerShell scripts pass AST syntax parsing without errors.
- [ ] Commit message follows `<type>(<scope>): <description>` format.
