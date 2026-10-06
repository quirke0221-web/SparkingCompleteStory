# `zip` Dependency Knowledge Base

* **Tool/Library:** `zip` `8.6.0` (latest stable release; `9.0.0-pre3` is unstable pre-release)
* **Repository:** [zip-rs/zip2](https://github.com/zip-rs/zip2)
* **License:** MIT
* **Minimum Supported Rust Version (MSRV):** 1.88 (Local environment is `1.98.1`)
* **Role in Project:** Native packaging of distributable mod release archives (`.zip`) in `complete-story-cli`.
* **Agent Skill:** [`.agents/skills/zip-archive-packager/`](../../../.agents/skills/zip-archive-packager/SKILL.md)

---

## Reference Documents in this Folder

| File | What it is | Trust level |
|---|---|---|
| [`raw-readme.md`](raw-readme.md) | Upstream README from `zip-rs/zip2` v8.6.0 | Primary source |
| [`archive-receipts.md`](archive-receipts.md) | Verbatim source receipts for `SimpleFileOptions`, directory compression, and Cargo configuration | Primary source |

---

## Correction Log

**2026-10-06:** An initial hypothesis assumed `zip 2.2` or `FileOptions`. Crates.io inspection proved `zip 8.6.0` is the latest stable release (transferred to `zip2`), and the API uses `SimpleFileOptions::default()`.

---

## Agent Skill Status
* **Status:** AUDITED & ACTIVE ([`.agents/skills/zip-archive-packager/SKILL.md`](../../../.agents/skills/zip-archive-packager/SKILL.md)).
