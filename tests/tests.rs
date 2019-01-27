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

// Tests that the Register enums
// correctly cast into u16 values
#[test]
fn registers_as_u16() {
    assert_eq!(RR0 as u16,   0);
    assert_eq!(RR1 as u16,   1);
    assert_eq!(RR2 as u16,   2);
    assert_eq!(RR3 as u16,   3);
    assert_eq!(RR4 as u16,   4);
    assert_eq!(RR5 as u16,   5);
    assert_eq!(RR6 as u16,   6);
    assert_eq!(RR7 as u16,   7);
    assert_eq!(RPC as u16,   8);
    assert_eq!(RCOND as u16, 9);
}

// Tests that the opcodes correctly
// cast into u16 values
#[test]
fn ops_as_u16() {
    assert_eq!(OP::BR as u16,   0);
    assert_eq!(OP::ADD as u16,  1);
    assert_eq!(OP::LD as u16,   2);
    assert_eq!(OP::ST as u16,   3);
    assert_eq!(OP::JSR as u16,  4);
    assert_eq!(OP::AND as u16,  5);
    assert_eq!(OP::LDR as u16,  6);
    assert_eq!(OP::STR as u16,  7);
    assert_eq!(OP::RTI as u16,  8);
    assert_eq!(OP::NOT as u16,  9);
    assert_eq!(OP::LDI as u16,  10);
    assert_eq!(OP::STI as u16,  11);
    assert_eq!(OP::JMP as u16,  12);
    assert_eq!(OP::RES as u16,  13);
    assert_eq!(OP::LEA as u16,  14);
    assert_eq!(OP::TRAP as u16, 15);
}

// Ensures the condition flags properly
// cast into u16 values
#[test]
fn condition_flags_as_u16() {
    assert_eq!(FL::POS as u16, 1);
    assert_eq!(FL::ZRO as u16, 2);
    assert_eq!(FL::NEG as u16, 4);
}

// Ensures that U16_MAX really is the
// maximum possible u16 value
#[test]
fn correct_u16_max() {
    assert_eq!(U16_MAX, 65536);      
}
