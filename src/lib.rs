// Broken off into a module, since it's unlikely
// there would be a need to import these,
// but it's public just in case the need arises
pub mod consts {
    // Maximum possible value to store within a u16
    pub const U16_MAX: usize = 1 << 16;

    // Default program counter state
    pub const PC_START: u16 = 0x3000;

    // Default state for the registers
    pub const DEFAULT_REGISTERS: [u16; 10] = [0,0,0,0,0,0,0,0,PC_START,0];

    // Default memory state (all 0's)
    pub const DEFAULT_MEMORY: [u16; U16_MAX] = [0; U16_MAX];
}

use consts::*;

// Represents the whole LC-3
pub struct LC3 {
    pub memory: [u16; U16_MAX],
    pub registers: [u16; 10]
}

impl LC3 {
    fn new() -> LC3 {
        LC3 {
            memory: DEFAULT_MEMORY,
            registers: DEFAULT_REGISTERS
        }
    }
}

// All registers
pub enum Register {
    RR0, RR1, RR2, RR3, RR4,
    RR5, RR6, RR7, RPC, RCOND
}

// All opcodes
pub enum OP {
    BR, ADD, LD, ST, JSR, AND, LDR, STR,
    RTI, NOT, LDI, STI, JMP, RES, LEA, TRAP
}

// Condition flags
pub enum FL {
    POS = 1 << 0,
    ZRO = 1 << 1,
    NEG = 1 << 2
}


#[cfg(test)]
pub mod tests {
    use crate::*;
    use Register::*;

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
}