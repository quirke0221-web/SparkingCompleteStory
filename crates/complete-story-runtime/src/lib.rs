//! Dragon Ball: Sparking! ZERO - Complete Story Native Runtime Hook
//!
//! Target: SparkingZERO-Win64-Shipping.exe (Unreal Engine 5.1.1)
//! Loaded as an .asi plugin by dsound.dll.

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
const SET_ACTIVE_SLOT_BOUNDS_RVA: usize = 0x24EF6D0;

static ORIGINAL_IS_PLAYABLE_IN_SAVE: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static ORIGINAL_SET_ACTIVE_SLOT_BOUNDS: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static QUERY_COUNT: AtomicU32 = AtomicU32::new(0);

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct FName {
    pub comparison_index: u32,
    pub number: u32,
}

pub struct CharacterInfo {
    pub key: &'static str,
    pub label: &'static str,
    pub index: AtomicU32,
}

static CHARACTERS: [CharacterInfo; 13] = [
    CharacterInfo { key: "0000_40", label: "0000_40 (Goku)", index: AtomicU32::new(0) },
    CharacterInfo { key: "0020_60", label: "0020_60 (Vegeta)", index: AtomicU32::new(0) },
    CharacterInfo { key: "0032_00", label: "0032_00 (Gohan)", index: AtomicU32::new(0) },
    CharacterInfo { key: "0050_00", label: "0050_00 (Piccolo)", index: AtomicU32::new(0) },
    CharacterInfo { key: "0040_00", label: "0040_00 (Trunks)", index: AtomicU32::new(0) },
    CharacterInfo { key: "0060_00", label: "0060_00 (Frieza)", index: AtomicU32::new(0) },
    CharacterInfo { key: "0070_00", label: "0070_00 (Goku Black)", index: AtomicU32::new(0) },
    CharacterInfo { key: "0080_30", label: "0080_30 (Krillin - Cut)", index: AtomicU32::new(0) },
    CharacterInfo { key: "0153_00", label: "0153_00 (Cell - Cut)", index: AtomicU32::new(0) },
    CharacterInfo { key: "0162_00", label: "0162_00 (Tien - Cut)", index: AtomicU32::new(0) },
    CharacterInfo { key: "0800_00", label: "0800_00 (Yamcha - Cut)", index: AtomicU32::new(0) },
    CharacterInfo { key: "0930_00", label: "0930_00 (Jiren)", index: AtomicU32::new(0) },
    CharacterInfo { key: "9999_00", label: "9999_00 (Complete Story)", index: AtomicU32::new(0) },
];

fn identify_key(idx: u32) -> (&'static str, bool) {
    for chara in &CHARACTERS {
        let stored = chara.index.load(Ordering::Relaxed);
        if stored != 0 && stored == idx {
            return (chara.label, chara.key == "9999_00");
        }
    }
    ("Unknown Key", false)
}

/// Native detour for Tier 3: IsCharacterPlayableInSave(const FKoratCharacterDataList* Key)
unsafe extern "C" fn detour_is_character_playable_in_save(key_ptr: *const FName) -> bool {
    let key = if !key_ptr.is_null() && (key_ptr as usize) >= 0x10000 {
        Some(*key_ptr)
    } else {
        None
    };

    let orig = ORIGINAL_IS_PLAYABLE_IN_SAVE.load(Ordering::Relaxed);
    let original_result = if !orig.is_null() {
        type FnIsPlayableInSave = unsafe extern "C" fn(*const FName) -> bool;
        let original: FnIsPlayableInSave = std::mem::transmute(orig);
        original(key_ptr)
    } else {
        false
    };

    if let Some(k) = key {
        let (label, is_custom) = identify_key(k.comparison_index);
        let count = QUERY_COUNT.fetch_add(1, Ordering::Relaxed);
        if count < 150 {
            if is_custom {
                logger::log_info(&format!(
                    "[TELEMETRY] Query #{} for '{}' (index={}) -> native={}, OVERRIDDEN to true",
                    count + 1, label, k.comparison_index, original_result
                ));
            } else {
                logger::log_info(&format!(
                    "[TELEMETRY] Query #{} for '{}' (index={}) -> native={}",
                    count + 1, label, k.comparison_index, original_result
                ));
            }
        }
        if is_custom {
            return true;
        }
    }

    original_result
}

/// Native detour for SetActiveSlotBounds(this, desired_count)
unsafe extern "C" fn detour_set_active_slot_bounds(this: *mut c_void, desired_count: u32) {
    logger::log_info(&format!(
        "[TELEMETRY] SetActiveSlotBounds called on manager {:p}: desired_count = {}",
        this, desired_count
    ));
    let orig = ORIGINAL_SET_ACTIVE_SLOT_BOUNDS.load(Ordering::Relaxed);
    if !orig.is_null() {
        type FnBounds = unsafe extern "C" fn(*mut c_void, u32);
        let original: FnBounds = std::mem::transmute(orig);
        original(this, desired_count);
    }
}

unsafe extern "system" fn init_thread(_: *mut c_void) -> u32 {
    logger::log_info("Native runtime thread initialized. Setting up telemetry hooks...");

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

    // 1. Hook Tier 3: IsCharacterPlayableInSave (RVA 0x2510ED0)
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

    // 2. Hook Active Slot Controller: SetActiveSlotBounds (RVA 0x24EF6D0)
    let bounds_addr = (base + SET_ACTIVE_SLOT_BOUNDS_RVA) as *mut c_void;
    let p_bounds = std::slice::from_raw_parts(bounds_addr as *const u8, 10);
    let expected_bounds = [0x48, 0x8B, 0xC4, 0x55, 0x53, 0x57, 0x48, 0x8D, 0x68, 0xA1];
    if p_bounds == expected_bounds {
        if let Ok(orig) = MinHook::create_hook(bounds_addr, detour_set_active_slot_bounds as _) {
            ORIGINAL_SET_ACTIVE_SLOT_BOUNDS.store(orig as *mut c_void, Ordering::Relaxed);
            logger::log_info(&format!(
                "SUCCESS: Hooked SetActiveSlotBounds at {:p}",
                bounds_addr
            ));
        }
    } else {
        logger::log_warn(&format!(
            "SetActiveSlotBounds prologue mismatch at {:p}: {:02X?}",
            bounds_addr, p_bounds
        ));
    }

    if let Err(e) = MinHook::enable_all_hooks() {
        logger::log_error(&format!("Failed to enable native hooks: {:?}", e));
        return 3;
    }

    // 3. Resolve FNames for all known character keys
    type FnFNameCtor = unsafe extern "C" fn(*mut FName, *const u16, u32) -> *mut FName;
    let fname_ctor: FnFNameCtor = std::mem::transmute(fname_ctor_addr);

    for attempt in 1..=60 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        let mut all_resolved = true;

        for chara in &CHARACTERS {
            if chara.index.load(Ordering::Relaxed) == 0 {
                let wide_str: Vec<u16> = format!("{}\0", chara.key).encode_utf16().collect();
                let mut fname_r = FName::default();
                fname_ctor(&mut fname_r, wide_str.as_ptr(), 1);

                if fname_r.comparison_index != 0 {
                    chara.index.store(fname_r.comparison_index, Ordering::SeqCst);
                    logger::log_info(&format!(
                        "Resolved '{}' -> ComparisonIndex={}",
                        chara.label, fname_r.comparison_index
                    ));
                } else {
                    all_resolved = false;
                }
            }
        }

        if all_resolved {
            logger::log_info(&format!("SUCCESS: All 13 character FNames resolved on attempt {}", attempt));
            return 0;
        }
    }

    logger::log_warn("Partial FName resolution timeout");
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
