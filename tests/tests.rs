use lc3vm::*;
use lc3vm::Register::*;

const NEGATIVE_NUM: u16 = 0b1111111111111111;
const POSITIVE_NUM: u16 = 0b0111111111111111;

// Test flag (FL) functionality
#[test]
fn flags() {
    // Test ZRO
    assert_eq!(
        FL::from(0) as u16,
        FL::ZRO as u16
    );

    // Test NEG
    assert_eq!(
        FL::from(NEGATIVE_NUM) as u16,
        FL::NEG as u16
    );

    // Test POS
    assert_eq!(
        FL::from(POSITIVE_NUM) as u16,
        FL::POS as u16
    );
}

// Test LC3's flag-setting functionality
#[test]
fn flag_setting() {
    let mut lc3 = LC3::new();
    
    // Test ZRO
    lc3.update_rcond(0);
    assert_eq!(
        lc3.registers[RCOND as usize],
        FL::ZRO as u16
    );

    // Test NEG
    lc3.registers[0] = NEGATIVE_NUM;
    lc3.update_rcond(0);
    assert_eq!(
        lc3.registers[RCOND as usize],
        FL::NEG as u16
    );

    // Test POS
    lc3.registers[0] = POSITIVE_NUM;
    lc3.update_rcond(0);
    assert_eq!(
        lc3.registers[RCOND as usize],
        FL::POS as u16
    );
}

// Ensures that U16_MAX really is the
// maximum possible u16 value
#[test]
fn correct_u16_max() {
    assert_eq!(U16_MAX, 65536);      
}
