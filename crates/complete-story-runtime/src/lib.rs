//! Dragon Ball: Sparking! ZERO - Complete Story Native Runtime Hook
//!
//! Target: SparkingZERO-Win64-Shipping.exe (Unreal Engine 5.1.1)
//! Loaded as an .asi plugin by dsound.dll.
//!
//! Tier 3 Architecture:
//! Detours Tier 3 IsCharacterPlayableInSave at RVA 0x2510ED0 directly,
//! satisfying the UI Carousel Widget Builder (0x24F602D), click checks,
//! and chapter launch validators with zero save file contamination.

pub mod logger;
pub mod pattern;

use minhook::MinHook;
use std::ffi::c_void;
use std::sync::atomic::{AtomicPtr, AtomicU32, Ordering};
use windows_sys::Win32::Foundation::{BOOL, HMODULE, TRUE};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleA;
use windows_sys::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

const FNAME_CONSTRUCTOR_FALLBACK_RVA: usize = 0x2CFD260;
const IS_CHARACTER_PLAYABLE_IN_SAVE_RVA: usize = 0x2510ED0;
const EXEC_IS_MODE_START_RVA: usize = 0x1EC3580;

static ORIGINAL_IS_PLAYABLE_IN_SAVE: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static ORIGINAL_EXEC_IS_MODE_START: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static ROUTE_KEY_INDEX: AtomicU32 = AtomicU32::new(0);
static OVERRIDE_COUNT: AtomicU32 = AtomicU32::new(0);

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct FName {
    pub comparison_index: u32,
    pub number: u32,
}

/// Native detour for Tier 3: IsCharacterPlayableInSave(const FKoratCharacterDataList* Key)
/// In x64 Windows ABI:
/// - rcx: Key pointer (points to 8-byte FName: [comparison_index: u32, number: u32])
/// - Returns: bool in AL (true if unlocked, false if locked)
unsafe extern "C" fn detour_is_character_playable_in_save(key_ptr: *const FName) -> bool {
    if !key_ptr.is_null() && (key_ptr as usize) >= 0x10000 {
        let key = *key_ptr;
        let route_idx = ROUTE_KEY_INDEX.load(Ordering::Relaxed);
        if route_idx != 0 && key.comparison_index == route_idx {
            let count = OVERRIDE_COUNT.fetch_add(1, Ordering::Relaxed);
            if count < 50 {
                logger::log_info(&format!(
                    "[TIER3_HOOK] #{} Overrode IsCharacterPlayableInSave for key (index={}) -> true",
                    count + 1, key.comparison_index
                ));
            }
            return true;
        }
    }

    let orig = ORIGINAL_IS_PLAYABLE_IN_SAVE.load(Ordering::Relaxed);
    if !orig.is_null() {
        type FnIsPlayableInSave = unsafe extern "C" fn(*const FName) -> bool;
        let original: FnIsPlayableInSave = std::mem::transmute(orig);
        original(key_ptr)
    } else {
        false
    }
}

/// Native detour for Tier 1: execIsModeStart (RVA 0x1EC3580)
/// Ensures confirmation dispatch immediately initiates chapter start.
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

    let fname_ctor_addr = pattern::resolve_function(
        base,
        FNAME_CONSTRUCTOR_FALLBACK_RVA,
        pattern::FNAME_CTOR_SIG,
        "FName::FName",
    );

    // 1. Install native MinHook detour on Tier 3: IsCharacterPlayableInSave (RVA 0x2510ED0)
    let playable_save_addr = (base + IS_CHARACTER_PLAYABLE_IN_SAVE_RVA) as *mut c_void;
    let p_playable = std::slice::from_raw_parts(playable_save_addr as *const u8, 12);
    let expected_p = [0x48, 0x89, 0x5C, 0x24, 0x08, 0x57, 0x48, 0x83, 0xEC, 0x20, 0x48, 0x8B];
    if p_playable == expected_p {
        if let Ok(orig) = MinHook::create_hook(playable_save_addr, detour_is_character_playable_in_save as _) {
            ORIGINAL_IS_PLAYABLE_IN_SAVE.store(orig as *mut c_void, Ordering::Relaxed);
            logger::log_info(&format!(
                "SUCCESS: Hooked Tier 3 IsCharacterPlayableInSave at {:p}",
                playable_save_addr
            ));
        }
    } else {
        logger::log_error(&format!(
            "Tier 3 prologue mismatch at {:p}: {:02X?}",
            playable_save_addr, p_playable
        ));
        return 2;
    }

    // 2. Install native MinHook detour on execIsModeStart (RVA 0x1EC3580)
    let exec_mode_start_addr = (base + EXEC_IS_MODE_START_RVA) as *mut c_void;
    let p_mode = std::slice::from_raw_parts(exec_mode_start_addr as *const u8, 6);
    if p_mode == [0x40, 0x53, 0x48, 0x83, 0xEC, 0x20] {
        if let Ok(orig) = MinHook::create_hook(exec_mode_start_addr, detour_exec_is_mode_start as _) {
            ORIGINAL_EXEC_IS_MODE_START.store(orig as *mut c_void, Ordering::Relaxed);
            logger::log_info(&format!(
                "SUCCESS: Hooked execIsModeStart at {:p}",
                exec_mode_start_addr
            ));
        }
    }

    if let Err(e) = MinHook::enable_all_hooks() {
        logger::log_error(&format!("Failed to enable native hooks: {:?}", e));
        return 3;
    }

    // 3. Asynchronously resolve FName ComparisonIndex for "0000_00"
    type FnFNameCtor = unsafe extern "C" fn(*mut FName, *const u16, u32) -> *mut FName;
    let fname_ctor: FnFNameCtor = std::mem::transmute(fname_ctor_addr);
    let wide_route: Vec<u16> = "0000_00\0".encode_utf16().collect();

    for attempt in 1..=60 {
        std::thread::sleep(std::time::Duration::from_millis(500));

        let mut fname_r = FName::default();
        fname_ctor(&mut fname_r, wide_route.as_ptr(), 1);

        if fname_r.comparison_index != 0 {
            ROUTE_KEY_INDEX.store(fname_r.comparison_index, Ordering::SeqCst);
            logger::log_info(&format!(
                "SUCCESS: Resolved FName for '0000_00' on attempt {}: ComparisonIndex={}",
                attempt, fname_r.comparison_index
            ));
            return 0;
        }
    }

    logger::log_warn("Timed out waiting for '0000_00' FName registration");
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
