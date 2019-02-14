mod lc3;
pub mod memory;
pub use lc3::LC3;

// Maximum possible value to store within a u16
pub const U16_MAX: usize = 1 << 16;

// All registers
pub enum R {
    R0, R1, R2, R3, R4,
    R5, R6, R7, PC, COND
}

// All opcodes
pub enum OP {
    BR, ADD, LD, ST, JSR, AND, LDR, STR,
    RTI, NOT, LDI, STI, JMP, RES, LEA, TRAP
}

// Condition flags
#[derive(PartialEq, Debug)]
pub enum FL {
    POS = 1,
    ZRO = 2,
    NEG = 4
}

// Makes it easier to check the RCOND
impl From<&LC3> for FL {
    fn from(lc3: &LC3) -> FL {
        match lc3.register[R::COND as usize] {
            1 => FL::POS,
            2 => FL::ZRO,
            4 => FL::NEG,
            _ => panic!("Attempt to create flag from invalid RCOND")
        }
    }
}

impl From<u16> for FL {
    fn from(register_val: u16) -> FL {
        if register_val == 0 {
            FL::ZRO
        } else if is_negative_u16(register_val) {
            FL::NEG
        } else {
            FL::POS
        }
    }
}

// Memory mapped registers
pub enum MR {
    KBSR = 0xFE00, // Keyboard status
    KBDR = 0xFE02 // Keyboard data
}

// Returns a bool based on whether the number given
// represents a negative number or not
pub const fn is_negative(x: u16, bit_count: u16) -> bool {
    x >> (bit_count - 1) == 1
}

// Gives the two's complement for a number
pub const fn complement(x: u16, bit_count: u16) -> u16 {
    x | (0xFFFF << bit_count)
}

// Extends a number out from bit_count to 16 bits while
// retaining its sign
pub fn sign_extend(x: u16, bit_count: u16) -> u16 {
    if is_negative(x, bit_count) {
        complement(x, bit_count)
    } else {
        x
    }
}

// Short for is_negative(x, 16)
pub const fn is_negative_u16(x: u16) -> bool {
    is_negative(x, 16)
}
