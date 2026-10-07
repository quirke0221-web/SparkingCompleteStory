//! Allocation-Free AOB Pattern Scanner for PE Memory Images
//!
//! Scans memory sections for opcode patterns to insulate against minor executable offsets.

use std::ffi::c_void;

pub const PROCESS_EVENT_SIG: &[Option<u8>] = &[
    Some(0x40), Some(0x55),                         // push rbp
    Some(0x56),                                     // push rsi
    Some(0x57),                                     // push rdi
    Some(0x41), Some(0x54),                         // push r12
    Some(0x41), Some(0x55),                         // push r13
    Some(0x41), Some(0x56),                         // push r14
    Some(0x41), Some(0x57),                         // push r15
    Some(0x48), Some(0x81), Some(0xEC), Some(0x10), Some(0x01), Some(0x00), Some(0x00), // sub rsp, 110h
    Some(0x48), Some(0x8D), Some(0x6C), Some(0x24), Some(0x30),                         // lea rbp, [rsp+30h]
    Some(0x48), Some(0x89), Some(0x9D), Some(0x38), Some(0x01), Some(0x00), Some(0x00), // mov [rbp+138h], rbx
];

pub const PROCESS_INTERNAL_SIG: &[Option<u8>] = &[
    Some(0x48), Some(0x89), Some(0x5C), Some(0x24), Some(0x08), // mov [rsp+8], rbx
    Some(0x48), Some(0x89), Some(0x6C), Some(0x24), Some(0x10), // mov [rsp+10h], rbp
    Some(0x48), Some(0x89), Some(0x74), Some(0x24), Some(0x18), // mov [rsp+18h], rsi
    Some(0x48), Some(0x89), Some(0x7C), Some(0x24), Some(0x20), // mov [rsp+20h], rdi
    Some(0x41), Some(0x56),                                     // push r14
    Some(0x48), Some(0x83), Some(0xEC), Some(0x30),             // sub rsp, 30h
    Some(0x48), Some(0x8B), Some(0x72), Some(0x10),             // mov rsi, [rdx+10h] (Stack.Node)
    Some(0x49), Some(0x8B), Some(0xE8),                         // mov rbp, r8 (Result)
];

pub const FNAME_CTOR_SIG: &[Option<u8>] = &[
    Some(0x48), Some(0x89), Some(0x5C), Some(0x24), Some(0x08), // mov [rsp+8], rbx
    Some(0x57),                                                 // push rdi
    Some(0x48), Some(0x83), Some(0xEC), Some(0x30),             // sub rsp, 30h
    Some(0x48), Some(0x8B), Some(0xD9),                         // mov rbx, rcx
    Some(0x48), Some(0x89), Some(0x54),                         // mov [rsp+...], rdx
];

/// Scans a byte buffer for a pattern with optional wildcards (None).
pub fn find_pattern(buffer: &[u8], pattern: &[Option<u8>]) -> Option<usize> {
    if pattern.is_empty() || buffer.len() < pattern.len() {
        return None;
    }

    let max_idx = buffer.len() - pattern.len();
    for i in 0..=max_idx {
        let mut matched = true;
        for (j, expected) in pattern.iter().enumerate() {
            if let Some(byte) = expected {
                if buffer[i + j] != *byte {
                    matched = false;
                    break;
                }
            }
        }
        if matched {
            return Some(i);
        }
    }
    None
}

/// Discovers the .text code section boundaries of a PE module in memory.
pub unsafe fn get_module_text_section(base_addr: usize) -> Option<(*const u8, usize)> {
    let dos_header = base_addr as *const u8;
    if *dos_header != b'M' || *dos_header.add(1) != b'Z' {
        return None;
    }

    let e_lfanew = *(dos_header.add(0x3C) as *const i32) as usize;
    let nt_headers = (base_addr + e_lfanew) as *const u8;

    let num_sections = *(nt_headers.add(0x06) as *const u16) as usize;
    let opt_header_size = *(nt_headers.add(0x14) as *const u16) as usize;

    let section_headers = nt_headers.add(0x18 + opt_header_size);

    for i in 0..num_sections {
        let sec = section_headers.add(i * 40);
        let name_slice = std::slice::from_raw_parts(sec, 8);
        if name_slice.starts_with(b".text") {
            let virtual_size = *(sec.add(8) as *const u32) as usize;
            let virtual_address = *(sec.add(12) as *const u32) as usize;
            return Some(((base_addr + virtual_address) as *const u8, virtual_size));
        }
    }

    None
}

/// Locates a function address using pattern scanning with RVA fallback.
pub unsafe fn resolve_function(
    base_addr: usize,
    fallback_rva: usize,
    pattern: &[Option<u8>],
    name: &str,
) -> *mut c_void {
    if let Some((text_ptr, text_size)) = get_module_text_section(base_addr) {
        let buffer = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_pattern(buffer, pattern) {
            let found_addr = text_ptr.add(offset) as *mut c_void;
            crate::logger::log_info(&format!(
                "Pattern scan found {} at {:p} (RVA 0x{:X})",
                name,
                found_addr,
                (found_addr as usize) - base_addr
            ));
            return found_addr;
        }
    }

    let fallback_addr = (base_addr + fallback_rva) as *mut c_void;
    crate::logger::log_warn(&format!(
        "Pattern scan failed for {}; using verified fallback RVA 0x{:X} ({:p})",
        name, fallback_rva, fallback_addr
    ));
    fallback_addr
}
