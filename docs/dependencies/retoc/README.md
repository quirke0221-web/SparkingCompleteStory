# `retoc` Dependency Knowledge Base

* **Tool:** `retoc` `0.1.5` (audited at commit [`885a8da`](https://github.com/trumank/retoc/tree/885a8dae740cb1ce1e41ff2e74f67f9f0c118237))
* **Author:** trumank
* **Repository:** [trumank/retoc](https://github.com/trumank/retoc)
* **Role in Project:** Unreal Engine 5 IoStore (`.utoc`/`.ucas`) extraction and packing
* **Agent Skill:** [`.agents/skills/retoc-iostore-packer/`](../../../.agents/skills/retoc-iostore-packer/SKILL.md)

---

## Reference Documents in this Folder

| File | What it is | Trust level |
|---|---|---|
| [`raw-readme.md`](raw-readme.md) | Upstream README, verbatim | Primary, but its `--help` block is older than the source |
| [`source-receipts-arguments.md`](source-receipts-arguments.md) | Verbatim source: global args, `to-legacy`/`to-zen`/`list` args, version tables | Primary |
| [`source-receipts-commands.md`](source-receipts-commands.md) | Verbatim source: subcommand list and minor command args | Primary |
| [`source-receipts-behavior.md`](source-receipts-behavior.md) | Verbatim source: how `to-legacy`, `to-zen`, `verify` behave | Primary |
| [`cli-reference.md`](cli-reference.md) | Readable CLI reference, every row line-cited to the receipts | Derived |
| [`sparking-zero-guide.md`](sparking-zero-guide.md) | Project recipes, labeled `[FACT]`/`[LOCAL]`/`[POLICY]`/`[HYPOTHESIS]` | Derived |

If a derived doc and a receipt disagree, the receipt wins. Fix the derived doc.

---

## Correction Log

**2026-10-05:** These claims from the first drafts were checked against source and were **[DISPROVEN]**:
1. "`to-zen` strictly requires `scriptobjects.bin`": it's optional upstream. It is required only by project policy (`Build-IoStore.ps1`).
2. "`verify` validates directory indexes": it only compares chunk hashes.
3. "`--filter` matches prefixes": it's a substring match.
4. `unpack` was described as raw chunk extraction: that's `unpack-raw`. Also added the missing commands: `asset-registry`, `print-script-objects`, `dump-test`, `--script-cell`.

Unverified claims were relabeled as `[HYPOTHESIS]`: single-`.utoc` input failing, building without `scriptobjects.bin` failing in-game. Unsupported statements were removed ("100GB+" archive size). Deployment rules were moved to the Unverum docs.

---

## Agent Skill Status
* **Status:** Draft skill written. Awaiting user audit.
