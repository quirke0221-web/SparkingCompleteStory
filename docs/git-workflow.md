# Complete Story: Git Workflow & Turn-Boundary Cadence

## 1. Purpose & Guiding Principles

This runbook defines the operational Git cadence for contributors and autonomous AI coding assistants (OpenAI Codex, Google Antigravity).

In an autonomous agent workflow:
1. **Git Commits as Checkpoints:** Git is not merely an archive for the end of the day—it is the active checkpoint system. Every verified code change, asset transformation, or bug fix must be captured in an atomic commit immediately.
2. **The Turn-Boundary Invariant:** An agent is **strictly FORBIDDEN** from ending a conversation turn or instructing the user to test without first committing and pushing all verified work.
3. **Anti-Drift Standard:** The working tree must remain clean (`git status` shows "nothing to commit, working tree clean") across agent turn boundaries. Accumulating uncommitted changes across turns is a critical governance violation.

---

## 2. The Atomic Micro-Commit Cadence

Whenever an agent or developer works on the codebase, execute this standard 4-step loop:

```text
┌────────────────────────────────────────────────────────┐
│ 1. IMPLEMENT & TEST                                    │
│    • Author the focused code or asset transformation.  │
│    • Validate syntax (e.g. PowerShell AST parser).     │
│    • Verify line ceilings (<= 300 lines max per file). │
└──────────────────────────┬─────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│ 2. STAGE SPECIFIC FILES                                │
│    • Stage only files belonging to this domain.        │
│    • Run: git add <files>                              │
│    • Check: git status (verify no untracked debris).   │
└──────────────────────────┬─────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│ 3. COMMIT WITH CONVENTIONAL STANDARDS                  │
│    • Format per docs/commit.md: <type>(<scope>): <msg> │
│    • Run: git commit -m "..."                          │
└──────────────────────────┬─────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│ 4. IMMEDIATE REMOTE PUSH                               │
│    • Push immediately: git push origin main            │
│    • Ensures remote collaborators are always in sync.  │
└────────────────────────────────────────────────────────┘
```

---

## 3. Turn-Boundary Definition of Done

An agent's turn is considered **DONE** only when all of the following conditions are met:
- [ ] No unstaged or uncommitted code changes exist in the working tree.
- [ ] New files strictly reside within their designated taxonomy folders (`CompleteStory/`, `crates/`, `docs/`, `.agents/`).
- [ ] Commit messages follow [`docs/commit.md`](commit.md).
- [ ] Changes are pushed to `origin main` so the user and remote teammates have access to the exact current state.

---

## 4. Rollback & Experimentation Protocol

When testing an experimental hypothesis:
1. **Never Manually Hack or Stash:** Do not create manual backup folders or stash diffs.
2. **Commit First:** Commit the working baseline before beginning an experiment.
3. **Rollback via Git:**
   * If an experiment fails: Use `git revert <commit-sha>` or `git restore <file>` to cleanly return to the proven baseline.
   * Never leave abandoned or half-broken code sitting uncommitted in the working tree.

---

## 5. Daily Collaboration Workflow

When a team member or agent resumes work:
1. **Pull Latest:** Run `git pull origin main` to ingest any updates made by collaborators.
2. **Check Clean State:** Run `git status` to confirm working tree is clean.
3. **Execute Task:** Implement, commit, and push per the micro-commit cadence above.
