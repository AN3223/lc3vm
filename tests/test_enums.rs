use lc3vm::*;

// Tests that the Register enums
// correctly cast into u16 values
#[test]
fn registers_as_u16() {
    assert_eq!(R::R0 as u16,   0);
    assert_eq!(R::R1 as u16,   1);
    assert_eq!(R::R2 as u16,   2);
    assert_eq!(R::R3 as u16,   3);
    assert_eq!(R::R4 as u16,   4);
    assert_eq!(R::R5 as u16,   5);
    assert_eq!(R::R6 as u16,   6);
    assert_eq!(R::R7 as u16,   7);
    assert_eq!(R::PC as u16,   8);
    assert_eq!(R::COND as u16, 9);
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