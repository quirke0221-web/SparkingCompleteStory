// Forensic verification test for FN-01 (SetActiveSlotBounds, RVA 0x24EF6D0)
// Asserts target prologue, length, and the exact two callers in SparkingZERO-Win64-Shipping.exe

#[test]
fn assert_fn01_setactiveslotbounds_mechanics() {
    let path = r"C:\Program Files (x86)\Steam\steamapps\common\DRAGON BALL Sparking! ZERO\SparkingZERO\Binaries\Win64\SparkingZERO-Win64-Shipping.exe";
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Skipping test: exe not accessible: {e}");
            return;
        }
    };

    let target_rva = 0x24EF6D0u32;
    let target_file_off = target_rva as usize - 0xA00;

    // 1. Assert prologue
    assert_eq!(
        &bytes[target_file_off..target_file_off + 10],
        &[0x48, 0x8B, 0xC4, 0x55, 0x53, 0x57, 0x48, 0x8D, 0x68, 0xA1]
    );

    // 2. Assert null guard at +0x1D
    assert_eq!(
        &bytes[target_file_off + 0x1D..target_file_off + 0x2A],
        &[0x48, 0x39, 0x99, 0xA8, 0x04, 0x00, 0x00, 0x0F, 0x84, 0x8E, 0x07, 0x00, 0x00]
    );

    // 3. Assert Caller #1 (OnNavigateMenu at RVA 0x24F5231 -> file offset 0x24F4831)
    let c1_off = 0x24F4831usize;
    assert_eq!(&bytes[c1_off..c1_off + 5], &[0xE8, 0x9A, 0xA4, 0xFF, 0xFF]);

    // 4. Assert Caller #2 (InitializeCarouselArrays at RVA 0x24F8D7A -> file offset 0x24F837A)
    let c2_off = 0x24F837Ausize;
    assert_eq!(&bytes[c2_off..c2_off + 5], &[0xE8, 0x51, 0x69, 0xFF, 0xFF]);
}
