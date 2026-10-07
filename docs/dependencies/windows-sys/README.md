# `windows-sys` Dependency Knowledge Base & Primary Receipts

* **What it is:** Zero-overhead raw FFI Win32 system bindings maintained by Microsoft.
* **Source Repository:** [`microsoft/windows-rs`](https://github.com/microsoft/windows-rs) / [`crates.io/crates/windows-sys`](https://crates.io/crates/windows-sys)
* **Pinned Version:** `0.59.0`
* **Target Layer:** Native dynamic runtime library (`crates/complete-story-runtime`).
* **Agent Skill:** [`.agents/skills/minhook-native-hooking/SKILL.md`](../../../.agents/skills/minhook-native-hooking/SKILL.md)

---

## 1. Verified Facts & Empirical Receipts

- `[FACT]` `windows-sys` exposes raw C-compatible definitions without COM wrappers or heavy type abstractions.
- `[FACT]` Features must be explicitly opted into in `Cargo.toml`. Required features for game process hooking:
  - `Win32_Foundation` (`BOOL`, `HMODULE`, `TRUE`, `FALSE`)
  - `Win32_System_LibraryLoader` (`GetModuleHandleA`, `GetProcAddress`)
  - `Win32_System_SystemServices` (`DLL_PROCESS_ATTACH`, `DLL_PROCESS_DETACH`)
  - `Win32_System_Memory` (`VirtualProtect`, `PAGE_EXECUTE_READWRITE`)
- `[FACT]` Pointers are represented as raw `*mut c_void` or typed handles (`HMODULE`), requiring standard Rust `unsafe` blocks.

---

## 2. Anti-Hallucination Guardrails

- `[POLICY]` Never import `windows::` when using `windows-sys::`. They are completely different crates with incompatible types (`windows` provides high-level COM bindings; `windows-sys` provides raw C ABI).
- `[POLICY]` Never block the loader lock inside `DllMain`. Any thread creation or delay must happen asynchronously outside `DllMain`.
