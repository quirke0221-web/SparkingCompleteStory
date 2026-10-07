---
name: minhook-native-hooking
description: Mandatory before ANY native runtime hook modifications or invocations in crates/complete-story-runtime. Governs MinHook x64 detours, AOB pattern scanning, and thread-safe Unreal Engine function overrides.
---

# Skill: MinHook Native Hooking (`complete-story-runtime`)

> **Status:** AUDITED & ACTIVE  
> **Toolchain Baseline:** `minhook 0.9.0` / `windows-sys 0.59.0` (x86_64-pc-windows-msvc)  
> **Crate Target:** `crates/complete-story-runtime`  
> **Reference Folder:** [`docs/dependencies/minhook/`](../../../docs/dependencies/minhook/README.md)  

---

## 1. Scope Boundaries

**Use this skill for:**
* Compiling and deploying the native runtime plugin `CompleteStory.asi`.
* Authoring or editing source files under `crates/complete-story-runtime/src/`.
* Installing, enabling, and managing Windows x64 function detours via `minhook`.
* Implementing patch-resilient AOB pattern scanning across the executable `.text` segment.
* Managing thread-safe overrides of Unreal Engine functions at the native ABI level.

**Do NOT use this skill for:**
* Writing UE4SS Lua scripts (use `ue4ss-runtime-scripting`).
* Authoring CLI commands or asset transformations (use `rust-pipeline-builder`).

---

## 2. Verified Golden Patterns

### 2.1 Detour Installation & Trampoline Calling
```rust
use minhook::MinHook;

// 1. Create hook
let original_ptr = unsafe { MinHook::create_hook(target_address, detour_function as _)? };

// 2. Enable hook
unsafe { MinHook::enable_all_hooks()? };

// 3. Trampoline invocation inside detour
type TargetFn = unsafe extern "C" fn(Context: *mut c_void, Stack: *mut c_void, Result: *mut c_void);
let original: TargetFn = unsafe { std::mem::transmute(original_ptr) };
original(context, stack, result);
```

### 2.2 AOB Pattern Scanning
* Always search within the executable's `.text` section bounded by its VirtualSize.
* Use wildcards (`?` or `0x00`) with mask arrays for relocatable instruction operands.
* Fall back to verified base RVAs with diagnostic logging if a pattern scan fails.

### 2.3 Hot-Path Detour Safety
* **Zero Allocations:** Never allocate heap memory (`String`, `Vec`, `GMalloc`) inside hot-path detours.
* **Atomic Matching:** Resolve string identifiers to 32-bit indices (`FName::ComparisonIndex`) once at startup; compare integers in hot callbacks.
* **Flush Diagnostics:** Always flush log output immediately (`file.flush()`) to survive process crashes.

---

## 3. Negative Constraints (Strictly Forbidden)

| Anti-Pattern | Failure Mode | Required Practice |
|---|---|---|
| Calling `FName::ToString` in hot detours | Memory leak in `GMalloc` (thousands/sec) | Cache 32-bit `ComparisonIndex` once |
| Hardcoding raw RVAs without pattern fallback | Mod breaks upon minor game updates | Use AOB pattern scanning over `.text` |
| Blocking loader lock in `DllMain` | Process deadlock at startup | Spawn initialization worker thread |
| Unflushed logger file buffers | Loss of critical logs during crash | Call `file.flush()` on every log entry |
