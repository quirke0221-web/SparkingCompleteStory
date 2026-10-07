use complete_story_runtime::pattern::find_pattern;
use complete_story_runtime::FName;


#[test]
fn test_exact_pattern_match() {
    let buffer = [0x90, 0x90, 0x48, 0x89, 0x5C, 0x24, 0x08, 0xC3];
    let pattern = [Some(0x48), Some(0x89), Some(0x5C), Some(0x24), Some(0x08)];
    assert_eq!(find_pattern(&buffer, &pattern), Some(2));
}

#[test]
fn test_wildcard_pattern_match() {
    let buffer = [0xAA, 0xBB, 0x48, 0x89, 0xFF, 0x24, 0x08, 0xCC];
    let pattern = [Some(0x48), Some(0x89), None, Some(0x24), Some(0x08)];
    assert_eq!(find_pattern(&buffer, &pattern), Some(2));
}

#[test]
fn test_pattern_not_found() {
    let buffer = [0x00, 0x01, 0x02, 0x03, 0x04];
    let pattern = [Some(0xFF), Some(0xFE)];
    assert_eq!(find_pattern(&buffer, &pattern), None);
}

#[test]
fn test_pattern_boundary_safety() {
    let buffer = [0x01];
    let pattern = [Some(0x01), Some(0x02)];
    assert_eq!(find_pattern(&buffer, &pattern), None);

    let empty_pattern: [Option<u8>; 0] = [];
    assert_eq!(find_pattern(&buffer, &empty_pattern), None);
}

#[test]
fn test_fname_layout_integrity() {
    assert_eq!(std::mem::size_of::<FName>(), 8);
    assert_eq!(std::mem::align_of::<FName>(), 4);
}

#[test]
fn test_process_internal_signature_match() {
    let real_ue5_bytes = [
        0x48, 0x89, 0x5C, 0x24, 0x08, 0x48, 0x89, 0x6C, 0x24, 0x10,
        0x48, 0x89, 0x74, 0x24, 0x18, 0x48, 0x89, 0x7C, 0x24, 0x20,
        0x41, 0x56, 0x48, 0x83, 0xEC, 0x30, 0x48, 0x8B, 0x72, 0x10,
        0x49, 0x8B, 0xE8, 0x48, 0x8B, 0x01,
    ];
    let result = find_pattern(&real_ue5_bytes, complete_story_runtime::pattern::PROCESS_INTERNAL_SIG);
    assert_eq!(result, Some(0));
}

#[test]
fn test_process_event_signature_match() {
    let real_ue5_bytes = [
        0x40, 0x55, 0x56, 0x57, 0x41, 0x54, 0x41, 0x55, 0x41, 0x56,
        0x41, 0x57, 0x48, 0x81, 0xEC, 0x10, 0x01, 0x00, 0x00, 0x48,
        0x8D, 0x6C, 0x24, 0x30, 0x48, 0x89, 0x9D, 0x38, 0x01, 0x00, 0x00,
    ];
    let result = find_pattern(&real_ue5_bytes, complete_story_runtime::pattern::PROCESS_EVENT_SIG);
    assert_eq!(result, Some(0));
}

#[test]
fn test_exec_functions_signature_and_prologue() {
    let path = r"C:\Program Files (x86)\Steam\steamapps\common\DRAGON BALL Sparking! ZERO\SparkingZERO\Binaries\Win64\SparkingZERO-Win64-Shipping.exe";
    if let Ok(bytes) = std::fs::read(path) {
        // execIsModeStart at RVA 0x1EC3580 -> file offset 0x1EC2B80
        let mode_start_off = 0x1EC2B80usize;
        let mode_start_prologue = &bytes[mode_start_off..mode_start_off + 6];
        assert_eq!(mode_start_prologue, &[0x40, 0x53, 0x48, 0x83, 0xEC, 0x20]);

        // execIsPlayable at RVA 0x1EC35B0 -> file offset 0x1EC2BB0
        let playable_off = 0x1EC2BB0usize;
        let playable_prologue = &bytes[playable_off..playable_off + 6];
        assert_eq!(playable_prologue, &[0x40, 0x53, 0x48, 0x83, 0xEC, 0x20]);

        // IsCharacterPlayableInSave at RVA 0x2510ED0 -> file offset 0x25104D0
        let playable_save_off = 0x25104D0usize;
        let playable_save_prologue = &bytes[playable_save_off..playable_save_off + 12];
        assert_eq!(
            playable_save_prologue,
            &[0x48, 0x89, 0x5C, 0x24, 0x08, 0x57, 0x48, 0x83, 0xEC, 0x20, 0x48, 0x8B]
        );

        // SetActiveSlotBounds at RVA 0x24EF6D0 -> file offset 0x24EECD0
        let bounds_off = 0x24EECD0usize;
        let bounds_prologue = &bytes[bounds_off..bounds_off + 10];
        assert_eq!(
            bounds_prologue,
            &[0x48, 0x8B, 0xC4, 0x55, 0x53, 0x57, 0x48, 0x8D, 0x68, 0xA1]
        );

        // CSManager Array Population Loop at RVA 0x24F8B2F -> file offset 0x24F812F
        let loop_off = 0x24F812Fusize;
        let loop_prologue = &bytes[loop_off..loop_off + 7];
        assert_eq!(
            loop_prologue,
            &[0x48, 0x8B, 0x35, 0x5A, 0x49, 0x21, 0x06]
        );
    }
}





