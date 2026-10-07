# `minhook` Dependency Knowledge Base & Primary Receipts

* **What it is:** Rust bindings to the minimalist x86/x64 API hooking library MinHook (`TsudaKageyu/minhook`).
* **Source Repository:** [`TsudaKageyu/minhook`](https://github.com/TsudaKageyu/minhook) / [`crates.io/crates/minhook`](https://crates.io/crates/minhook)
* **Pinned Version:** `0.9.0`
* **Target Layer:** Native dynamic runtime library (`crates/complete-story-runtime`, deployed as `CompleteStory.asi`).
* **Agent Skill:** [`.agents/skills/minhook-native-hooking/SKILL.md`](../../../.agents/skills/minhook-native-hooking/SKILL.md)

---

## 1. Verified Facts & Empirical Receipts

- `[FACT]` MinHook is a Windows-exclusive x86/x64 trampoline hooking library that overwrites function prologues with a 14-byte 64-bit indirect jump (`FF 25 00 00 00 00 [target]`).
- `[FACT]` `MinHook::create_hook(target as _, detour as _)` creates a dormant detour and returns a pointer to the original function trampoline.
- `[FACT]` `MinHook::enable_all_hooks()` or `MinHook::enable_hook(target as _)` activates the detour atomically using `VirtualProtect` to swap memory protections.
- `[FACT]` Transmuting the original trampoline address requires `unsafe { std::mem::transmute::<_, TargetFnType>(original_addr) }`.
- `[FACT]` Detour functions must match the exact ABI of the hooked target (`extern "C"` or `extern "system"`). Mismatched calling conventions corrupt the RSP stack register and trigger immediate access violations (`0xC0000005`).

---

## 2. Anti-Hallucination Guardrails

- `[POLICY]` Never attempt to hook unaligned addresses or addresses outside executable PE memory segments (`.text`).
- `[POLICY]` In multithreaded game loops, always call original trampolines before or after mutating registers, and never allocate memory inside hot-path detours.
- `[POLICY]` Always guard detour pointers with null checks before dereferencing stack or register buffers.
