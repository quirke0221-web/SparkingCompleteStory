//! Dragon Ball: Sparking! ZERO - Complete Story Native Runtime Hook
//!
//! Target: SparkingZERO-Win64-Shipping.exe (Unreal Engine 5.1.1)
//! Loaded as an .asi plugin by dsound.dll.
//!
//! Responsibilities:
//! 1. Detours UObject::ProcessInternal at native ABI level via MinHook.
//! 2. Extracts UFunction from Stack.Node at offset 0x10 of FFrame.
//! 3. Overrides ReturnValue to true for IsPlayable and IsModeStart.

pub mod logger;
pub mod pattern;

use minhook::MinHook;
use std::ffi::c_void;
use std::sync::atomic::{AtomicPtr, AtomicU32, Ordering};
use windows_sys::Win32::Foundation::{BOOL, HMODULE, TRUE};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleA;
use windows_sys::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

const PROCESS_INTERNAL_FALLBACK_RVA: usize = 0x2E832D0;
const FNAME_CONSTRUCTOR_FALLBACK_RVA: usize = 0x2CFD260;
const EXEC_IS_PLAYABLE_RVA: usize = 0x1EC35B0;
const EXEC_IS_MODE_START_RVA: usize = 0x1EC3580;

static ORIGINAL_PROCESS_INTERNAL: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static ORIGINAL_EXEC_IS_PLAYABLE: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static ORIGINAL_EXEC_IS_MODE_START: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static IS_PLAYABLE_INDEX: AtomicU32 = AtomicU32::new(0);
static IS_MODE_START_INDEX: AtomicU32 = AtomicU32::new(0);
static OVERRIDE_COUNT: AtomicU32 = AtomicU32::new(0);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FName {
    pub comparison_index: u32,
    pub number: u32,
}

/// Native detour for UObject::ProcessInternal(FFrame& Stack, void* const Result)
/// In x64 Windows ABI:
/// - rcx: this (UObject* Context)
/// - rdx: Stack (FFrame&) -> Stack.Node (UFunction*) is at offset 0x10 (mov rsi, [rdx+10h])
/// - r8:  Result (void* const Result)
unsafe extern "C" fn detour_process_internal(
    context: *mut c_void,
    stack: *mut c_void,
    result: *mut c_void,
) {
    let orig = ORIGINAL_PROCESS_INTERNAL.load(Ordering::Relaxed);
    if orig.is_null() {
        return;
    }

    type FnProcessInternal = unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void);
    let original: FnProcessInternal = std::mem::transmute(orig);

    // 1. Call original function execution first
    original(context, stack, result);

    if stack.is_null() || (stack as usize) < 0x10000 {
        return;
    }

    let is_playable = IS_PLAYABLE_INDEX.load(Ordering::Relaxed);
    let is_mode_start = IS_MODE_START_INDEX.load(Ordering::Relaxed);
    if is_playable == 0 && is_mode_start == 0 {
        return;
    }

    // 2. Extract UFunction* from Stack.Node (offset 0x10 of FFrame)
    let ufunc = *(stack.add(0x10) as *const *mut c_void);
    if ufunc.is_null() || (ufunc as usize) < 0x10000 {
        return;
    }

    // 3. UFunction inherits UObjectBase: NamePrivate.ComparisonIndex is at offset 0x18
    let comp_idx = *(ufunc.add(0x18) as *const u32);
    if comp_idx == is_playable || comp_idx == is_mode_start {
        if !result.is_null() {
            *(result as *mut bool) = true;
            let count = OVERRIDE_COUNT.fetch_add(1, Ordering::Relaxed);
            if count < 50 {
                let fn_tag = if comp_idx == is_playable {
                    "IsPlayable"
                } else {
                    "IsModeStart"
                };
                logger::log_info(&format!(
                    "[HOTPATH] #{} Overrode {} (index={}) result -> true",
                    count + 1, fn_tag, comp_idx
                ));
            }
        }
    }
}

/// Native detour for execIsPlayable (RVA 0x1EC35B0)
unsafe extern "C" fn detour_exec_is_playable(
    context: *mut c_void,
    stack: *mut c_void,
    result: *mut c_void,
) {
    let orig = ORIGINAL_EXEC_IS_PLAYABLE.load(Ordering::Relaxed);
    if !orig.is_null() {
        type FnNative = unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void);
        let original: FnNative = std::mem::transmute(orig);
        original(context, stack, result);
    }
    if !result.is_null() {
        *(result as *mut bool) = true;
        let count = OVERRIDE_COUNT.fetch_add(1, Ordering::Relaxed);
        if count < 50 {
            logger::log_info(&format!("[EXEC_HOOK] Overrode execIsPlayable -> true (#{})", count + 1));
        }
    }
}

/// Native detour for execIsModeStart (RVA 0x1EC3580)
unsafe extern "C" fn detour_exec_is_mode_start(
    context: *mut c_void,
    stack: *mut c_void,
    result: *mut c_void,
) {
    let orig = ORIGINAL_EXEC_IS_MODE_START.load(Ordering::Relaxed);
    if !orig.is_null() {
        type FnNative = unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void);
        let original: FnNative = std::mem::transmute(orig);
        original(context, stack, result);
    }
    if !result.is_null() {
        *(result as *mut bool) = true;
        let count = OVERRIDE_COUNT.fetch_add(1, Ordering::Relaxed);
        if count < 50 {
            logger::log_info(&format!("[EXEC_HOOK] Overrode execIsModeStart -> true (#{})", count + 1));
        }
    }
}

unsafe extern "system" fn init_thread(_: *mut c_void) -> u32 {
    logger::log_info("Native runtime thread initialized. Scanning memory...");

    let base = GetModuleHandleA(std::ptr::null()) as usize;
    if base == 0 {
        logger::log_error("Failed to obtain module handle for main executable");
        return 1;
    }

    // Resolve ProcessInternal function address via pattern scanning with RVA fallback
    let proc_addr = pattern::resolve_function(
        base,
        PROCESS_INTERNAL_FALLBACK_RVA,
        pattern::PROCESS_INTERNAL_SIG,
        "ProcessInternal",
    );

    let fname_ctor_addr = pattern::resolve_function(
        base,
        FNAME_CONSTRUCTOR_FALLBACK_RVA,
        pattern::FNAME_CTOR_SIG,
        "FName::FName",
    );

    // Verify ProcessInternal prologue (48 89 5C 24 08 -> mov [rsp+8], rbx)
    let prologue = std::slice::from_raw_parts(proc_addr as *const u8, 5);
    if prologue != [0x48, 0x89, 0x5C, 0x24, 0x08] {
        logger::log_error(&format!(
            "ProcessInternal prologue mismatch at {:p}: {:02X?}",
            proc_addr, prologue
        ));
        return 2;
    }

    // Install native MinHook detour on ProcessInternal
    if let Ok(original) = MinHook::create_hook(proc_addr, detour_process_internal as _) {
        ORIGINAL_PROCESS_INTERNAL.store(original as *mut c_void, Ordering::Relaxed);
        logger::log_info(&format!("SUCCESS: Hooked ProcessInternal at {:p}", proc_addr));
    }

    // Install native MinHook detours on execIsPlayable and execIsModeStart
    let exec_playable_addr = (base + EXEC_IS_PLAYABLE_RVA) as *mut c_void;
    let p_playable = std::slice::from_raw_parts(exec_playable_addr as *const u8, 6);
    if p_playable == [0x40, 0x53, 0x48, 0x83, 0xEC, 0x20] {
        if let Ok(orig) = MinHook::create_hook(exec_playable_addr, detour_exec_is_playable as _) {
            ORIGINAL_EXEC_IS_PLAYABLE.store(orig as *mut c_void, Ordering::Relaxed);
            logger::log_info(&format!("SUCCESS: Hooked execIsPlayable at {:p}", exec_playable_addr));
        }
    }

    let exec_mode_start_addr = (base + EXEC_IS_MODE_START_RVA) as *mut c_void;
    let p_mode = std::slice::from_raw_parts(exec_mode_start_addr as *const u8, 6);
    if p_mode == [0x40, 0x53, 0x48, 0x83, 0xEC, 0x20] {
        if let Ok(orig) = MinHook::create_hook(exec_mode_start_addr, detour_exec_is_mode_start as _) {
            ORIGINAL_EXEC_IS_MODE_START.store(orig as *mut c_void, Ordering::Relaxed);
            logger::log_info(&format!("SUCCESS: Hooked execIsModeStart at {:p}", exec_mode_start_addr));
        }
    }

    if let Err(e) = MinHook::enable_all_hooks() {
        logger::log_error(&format!("Failed to enable native hooks: {:?}", e));
        return 3;
    }

    // Asynchronously resolve FName ComparisonIndex
    type FnFNameCtor = unsafe extern "C" fn(*mut FName, *const u16, u32) -> *mut FName;
    let fname_ctor: FnFNameCtor = std::mem::transmute(fname_ctor_addr);

    let wide_play: Vec<u16> = "IsPlayable\0".encode_utf16().collect();
    let wide_start: Vec<u16> = "IsModeStart\0".encode_utf16().collect();

    for attempt in 1..=60 {
        std::thread::sleep(std::time::Duration::from_millis(500));

        let mut fname_p = FName::default();
        let mut fname_s = FName::default();

        fname_ctor(&mut fname_p, wide_play.as_ptr(), 1);
        fname_ctor(&mut fname_s, wide_start.as_ptr(), 1);

        if fname_p.comparison_index != 0 {
            IS_PLAYABLE_INDEX.store(fname_p.comparison_index, Ordering::SeqCst);
            IS_MODE_START_INDEX.store(fname_s.comparison_index, Ordering::SeqCst);
            logger::log_info(&format!(
                "SUCCESS: Resolved FNames on attempt {}: IsPlayable={}, IsModeStart={}",
                attempt, fname_p.comparison_index, fname_s.comparison_index
            ));
            return 0;
        }
    }

    logger::log_warn("Timed out waiting for IsPlayable FName registration");
    0
}

#[no_mangle]
pub unsafe extern "system" fn DllMain(
    _hinst_dll: HMODULE,
    fdw_reason: u32,
    _lpv_reserved: *mut c_void,
) -> BOOL {
    if fdw_reason == DLL_PROCESS_ATTACH {
        std::thread::spawn(|| {
            init_thread(std::ptr::null_mut());
        });
    }
    TRUE
}
